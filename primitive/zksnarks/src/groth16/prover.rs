use super::constraint::Constraint;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use poly_commit::{Coefficients, Fft, PointsValue};
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

        let a_poly = fft.idft(PointsValue(cs.a.clone()));
        let b_poly = fft.idft(PointsValue(cs.b.clone()));
        let c_poly = fft.idft(PointsValue(cs.c.clone()));
        let t_poly = fft.idft(PointsValue(
            (1..=cs.m())
                .map(|i| P::ScalarField::from(i as u64))
                .collect(),
        ));

        // println!("Constr = {:#?}", cs.constraints);
        // println!("A = {:?}", cs.a);
        // println!("B = {:?}", cs.b);
        // println!("C = {:?}", cs.c);
        // println!("A_poly = {:?}", a_poly);
        // println!("B_poly = {:?}", b_poly);
        // println!("C_poly = {:?}", c_poly);

        for i in 1..=cs.m() {
            let x = P::ScalarField::from(i as u64);
            let a_val = a_poly.evaluate(&x);
            let b_val = b_poly.evaluate(&x);
            let c_val = c_poly.evaluate(&x);
            // println! {"{:?} * {:?} == {:?}", a_val, b_val, c_val};
        }

        let left = Coefficients::new(naive_multiply(a_poly.0, b_poly.0)) - c_poly;
        // println!("Left = {left:?}");
        let mut h = left;

        for at in (1..=cs.m()).map(|i| P::ScalarField::from(i as u64)) {
            // println!("H = {h:?}");
            h = h.divide(&at); // (x - at) 1..n
        }

        // Ok((1..=cs.m())
        //     .map(|i| P::ScalarField::from(i as u64))
        //     .all(|at| left.evaluate(&at) == h.evaluate(&at) * t_poly.evaluate(&at)))

        Ok(cs.constraints.iter().all(|constraint| {
            let (a, b, c) = constraint.evaluate(&cs.instance, &cs.witness);
            a * b == c
        }))
    }
}
