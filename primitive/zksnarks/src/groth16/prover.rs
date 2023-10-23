use super::constraint::Constraint;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use poly_commit::{Fft, PointsValue};
use zkstd::common::{vec, Pairing, PrimeField, TwistedEdwardsCurve, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    pub constraints: Vec<Constraint<<P::JubjubAffine as TwistedEdwardsCurve>::Range>>,
}

fn naive_multiply<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
    assert_eq!(a.len(), b.len());
    let mut c = vec![F::zero(); a.len() + b.len()];
    a.iter().enumerate().for_each(|(i_a, coeff_a)| {
        b.iter().enumerate().for_each(|(i_b, coeff_b)| {
            c[i_a + i_b] += *coeff_a * *coeff_b;
        })
    });
    c
}

impl<P: Pairing> Prover<P> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C>(&mut self, circuit: C) -> Result<bool, Error>
    where
        C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>,
    {
        let mut cs = Groth16::<P::JubjubAffine>::initialize();
        circuit.synthesize(&mut cs)?;

        cs.eval_constraints(); // -> a, b, c

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();

        let fft = Fft::<P::ScalarField>::new(k as usize);

        let h = {
            let a = fft.idft(PointsValue(cs.a.clone()));
            let b = fft.idft(PointsValue(cs.b.clone()));
            let c = fft.idft(PointsValue(cs.c.clone()));

            let mut a = fft.coset_dft(a);
            let b = fft.coset_dft(b);
            let c = fft.coset_dft(c);

            // println!("A = {:?}", a);
            // println!("B = {:?}", b);
            // println!("C = {:?}", c);

            a = fft.points_mul(a, b);
            a = &a - &c;

            a = fft.divide_by_z_on_coset(a);

            let mut a = fft.coset_idft(a);
            a.0.truncate(a.len() - 1);

            // println!("A = {:?}", a);
            // println!("B = {:?}", b);
            // println!("C = {:?}", c);

            a
            // a.mul_assign(&b);
            // drop(b);
            // a.sub_assign(&worker, &c);
            // drop(c);
            // a.divide_by_z_on_coset(&worker);
            // a.icoset_fft(&worker);
            // let mut a = a.into_coeffs();
            // let a_len = a.len() - 1;
            // a.truncate(a_len);
            // // TODO: parallelize if it's even helpful
            // let a = Arc::new(a.into_iter().map(|s| s.0.into()).collect::<Vec<_>>());
            //
            // multiexp(&worker, params.get_h(a.len())?, FullDensity, a)
        };

        // println!("H = {:?}", h);
        // println!("Constr = {:#?}", cs.constraints);
        // println!("A = {:?}", cs.a);
        // println!("B = {:?}", cs.b);
        // println!("C = {:?}", cs.c);
        // println!("A_poly = {:?}", a_poly);
        // println!("B_poly = {:?}", b_poly);
        // println!("C_poly = {:?}", c_poly);

        Ok(cs.constraints.iter().all(|constraint| {
            let (a, b, c) = constraint.evaluate(&cs.instance, &cs.witness);
            a * b == c
        }))
    }
}
