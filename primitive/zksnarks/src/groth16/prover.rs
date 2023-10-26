use super::constraint::Constraint;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use poly_commit::{Fft, PointsValue};
use rand::rngs::OsRng;
use zkstd::common::{Group, Pairing, TwistedEdwardsCurve, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    pub constraints: Vec<Constraint<<P::JubjubAffine as TwistedEdwardsCurve>::Range>>,
}

impl<P: Pairing> Prover<P> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C>(&mut self, circuit: C) -> Result<bool, Error>
    where
        C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>,
    {
        let mut cs = Groth16::<P::JubjubAffine>::initialize();
        circuit.synthesize(&mut cs)?;

        cs.eval_constraints();

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();

        let fft = Fft::<P::ScalarField>::new(k as usize);

        let (left, h) = {
            let a = fft.idft(PointsValue(cs.a.clone()));
            let b = fft.idft(PointsValue(cs.b.clone()));
            let c = fft.idft(PointsValue(cs.c.clone()));

            let mut a = fft.coset_dft(a);
            let b = fft.coset_dft(b);
            let c = fft.coset_dft(c);

            a = fft.points_mul(a, b);
            a = &a - &c;

            let left = fft.coset_idft(a.clone());
            a = fft.divide_by_z_on_coset(a);
            let mut a = fft.coset_idft(a);
            a.0.truncate(fft.size() - 1);

            (left, a)
        };

        let point = P::ScalarField::random(OsRng);
        let a_eval = left.evaluate(&point);
        let h_eval = h.evaluate(&point);
        let t_eval = fft.z_on_coset();
        let right: P::ScalarField = h_eval * t_eval;

        assert_eq!(a_eval, right);

        Ok(cs.constraints.iter().all(|constraint| {
            let (a, b, c) = constraint.evaluate(&cs.instance, &cs.witness);
            a * b == c
        }))
    }
}
