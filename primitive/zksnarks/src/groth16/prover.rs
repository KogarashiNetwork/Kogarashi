use super::constraint::Constraint;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use poly_commit::{Fft, PointsValue};
use zkstd::common::{vec, Pairing, PrimeField, Ring, TwistedEdwardsCurve, Vec};

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

        println!("A = {:?}", cs.a);
        println!("B = {:?}", cs.b);
        println!("C = {:?}", cs.c);

        println!("inputs = {:#?}", cs.instance);
        println!("aux = {:#?}", cs.witness);

        let (left, h) = {
            let a = fft.idft(PointsValue(cs.a.clone()));
            let b = fft.idft(PointsValue(cs.b.clone()));
            let c = fft.idft(PointsValue(cs.c.clone()));
            println!("A = {:?}", a);
            println!("B = {:?}", b);
            println!("C = {:?}", c);

            let mut a = fft.coset_dft(a);
            let b = fft.coset_dft(b);
            let c = fft.coset_dft(c);

            println!("A_coset = {:?}", a);
            println!("B_coset = {:?}", b);
            println!("C_coset = {:?}", c);

            a = fft.points_mul(a, b);
            a = &a - &c;

            let left = fft.coset_idft(a.clone());

            a = fft.divide_by_z_on_coset(a);

            let mut a = fft.coset_idft(a);
            a.0.truncate(a.len() - 1);

            (left, a)
        };

        println!("H = {:?}", h);
        println!("Left_coeff = {:?}", left);

        let point = P::ScalarField::from(35);
        let a_eval = left.evaluate(&point);
        let h_eval = h.evaluate(&point);
        let t_eval = fft.z_on_coset();

        let right: P::ScalarField = h_eval * t_eval;

        println!("Left = {:?}", a_eval);
        println!("H = {:?}", h_eval);
        println!("T = {:?}", t_eval);
        println!("Right = {:?}", right);
        assert_eq!(a_eval, right);
        // assert!(left.evaluate(&point) == h.evaluate(&point) * fft.z_on_coset());

        // A, B, C - > x = 1..cs.m() -> R1CS_Eval(instance, witness)

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
