#![allow(dead_code)]
mod constraint;
mod expression;
mod params;
mod prover;
mod util;
mod verifier;

pub(crate) mod curves;
pub(crate) mod error;
mod key;
pub mod wire;

use crate::constraint_system::ConstraintSystem;

use constraint::Constraint;
use curves::EdwardsExpression;
use expression::Expression;
use wire::{Index, Wire};
use zkstd::common::{vec, Group, Ring, TwistedEdwardsAffine, Vec};

#[derive(Debug)]
pub struct Groth16<C: TwistedEdwardsAffine> {
    constraints: Vec<Constraint<C::Range>>,
    a: Vec<C::Range>,
    b: Vec<C::Range>,
    c: Vec<C::Range>,
    pub(crate) instance: Vec<(Wire, C::Range)>,
    pub(crate) witness: Vec<(Wire, C::Range)>,
}

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> for Groth16<C> {
    type Wire = Wire;
    type Constraints = Vec<Constraint<C::Range>>;

    fn initialize() -> Self {
        Self::new()
    }

    fn m(&self) -> usize {
        self.constraints.len()
    }

    fn instance(&self) -> Vec<<C>::Range> {
        Vec::new()
    }

    fn constraints(&self) -> Self::Constraints {
        self.constraints.clone()
    }

    fn alloc_instance(&mut self, instance: C::Range) -> Wire {
        let wire = self.public_wire();
        self.instance.push((wire, instance));
        wire
    }

    fn alloc_witness(&mut self, witness: C::Range) -> Wire {
        let wire = self.private_wire();
        self.witness.push((wire, witness));
        wire
    }
}

impl<C: TwistedEdwardsAffine> Groth16<C> {
    fn new() -> Self {
        Self {
            constraints: Vec::new(),
            a: vec![],
            b: vec![],
            c: vec![],
            instance: [(Wire::ONE, C::Range::one())].into_iter().collect(),
            witness: vec![],
        }
    }

    fn inputs_iter(
        &self,
    ) -> (
        Vec<Vec<(C::Range, usize)>>,
        Vec<Vec<(C::Range, usize)>>,
        Vec<Vec<(C::Range, usize)>>,
    ) {
        let mut at = vec![vec![]; self.instance_len()];
        let mut bt = vec![vec![]; self.instance_len()];
        let mut ct = vec![vec![]; self.instance_len()];
        for (i, Constraint { a, b, c }) in self.constraints.iter().enumerate() {
            a.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Input(_)))
                .for_each(|(&w, coeff)| {
                    at[*w.get_unchecked()].push((*coeff, i));
                });
            b.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Input(_)))
                .for_each(|(&w, coeff)| {
                    bt[*w.get_unchecked()].push((*coeff, i));
                });
            c.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Input(_)))
                .for_each(|(&w, coeff)| {
                    ct[*w.get_unchecked()].push((*coeff, i));
                });
        }

        (at, bt, ct)
    }

    fn aux_iter(
        &self,
    ) -> (
        Vec<Vec<(C::Range, usize)>>,
        Vec<Vec<(C::Range, usize)>>,
        Vec<Vec<(C::Range, usize)>>,
    ) {
        let mut at = vec![vec![]; self.witness_len()];
        let mut bt = vec![vec![]; self.witness_len()];
        let mut ct = vec![vec![]; self.witness_len()];
        for (i, Constraint { a, b, c }) in self.constraints.iter().enumerate() {
            a.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Aux(_)))
                .for_each(|(&w, coeff)| {
                    at[*w.get_unchecked()].push((*coeff, i));
                });
            b.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Aux(_)))
                .for_each(|(&w, coeff)| {
                    bt[*w.get_unchecked()].push((*coeff, i));
                });
            c.coefficients()
                .iter()
                .filter(|(&w, _)| matches!(w.get_unchecked(), Index::Aux(_)))
                .for_each(|(&w, coeff)| {
                    ct[*w.get_unchecked()].push((*coeff, i));
                });
        }

        (at, bt, ct)
    }

    fn eval_constraints(&mut self) {
        for x in self.constraints.iter() {
            let (a, b, c) = x.evaluate(&self.instance, &self.witness);
            self.a.push(a);
            self.b.push(b);
            self.c.push(c);
        }
    }

    fn instance_len(&self) -> usize {
        self.instance.len()
    }
    fn witness_len(&self) -> usize {
        self.witness.len()
    }

    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.instance.len();
        Wire::new_unchecked(Index::Input(index))
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.witness.len();
        Wire::new_unchecked(Index::Aux(index))
    }

    pub fn append_edwards_expression(
        &mut self,
        x: Expression<C::Range>,
        y: Expression<C::Range>,
    ) -> EdwardsExpression<C::Range, C> {
        let x_squared = self.product(&x, &x);
        let y_squared = self.product(&y, &y);
        let x_squared_y_squared = self.product(&x_squared, &y_squared);

        self.assert_equal(
            &y_squared,
            &(Expression::one() + x_squared_y_squared * C::PARAM_D + &x_squared),
        );

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Assert that x * y = z;
    pub fn assert_product(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
        z: &Expression<C::Range>,
    ) {
        self.constraints.push(Constraint {
            a: x.clone(),
            b: y.clone(),
            c: z.clone(),
        });
    }

    // Assert that x + y = z;
    pub fn assert_sum(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
        z: &Expression<C::Range>,
    ) {
        self.constraints.push(Constraint {
            a: x + y,
            b: Expression::from(Wire::ONE),
            c: z.clone(),
        });
    }

    /// Assert that x == y.
    pub fn assert_equal(&mut self, x: &Expression<C::Range>, y: &Expression<C::Range>) {
        self.assert_product(x, &Expression::one(), y);
    }

    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn product(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
    ) -> Expression<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value =
            x.evaluate(&self.instance, &self.witness) * y.evaluate(&self.instance, &self.witness);
        let product = self.alloc_witness(product_value);
        let product_exp = Expression::from(product);
        self.assert_product(x, y, &product_exp);

        product_exp
    }

    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn sum(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
    ) -> Expression<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let sum_value =
            x.evaluate(&self.instance, &self.witness) + y.evaluate(&self.instance, &self.witness);
        let sum = self.alloc_witness(sum_value);
        let sum_exp = Expression::from(sum);
        self.assert_sum(x, y, &sum_exp);
        sum_exp
    }

    /// Returns `1 / x`, assuming `x` is non-zero. If `x` is zero, the gadget will not be
    /// satisfiable.
    pub fn inverse(&mut self, x: &Expression<C::Range>) -> Expression<C::Range> {
        let x_value = x.evaluate(&self.instance, &self.witness);
        let inverse_value = x_value.invert().expect("Can't find an inverse element");
        let x_inv = self.alloc_witness(inverse_value);

        let x_inv_expression = Expression::from(x_inv);
        self.assert_product(x, &x_inv_expression, &Expression::one());

        x_inv_expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::constraint_system::ConstraintSystem;
    use crate::error::Error;
    use crate::groth16::key::Groth16Key;
    use crate::groth16::params::Groth16Params;
    use crate::keypair::Keypair;
    use crate::public_params::PublicParameters;
    use bls_12_381::Fr as BlsScalar;
    use ec_pairing::TatePairing;
    use expression::Expression;
    use jub_jub::JubjubAffine;
    use rand::rngs::OsRng;

    #[test]
    fn circuit_to_r1cs() {
        #[derive(Debug)]
        pub struct DummyCircuit {
            x: BlsScalar,
            y: BlsScalar,
        }

        impl DummyCircuit {
            pub fn new(x: BlsScalar, y: BlsScalar) -> Self {
                Self { x, y }
            }
        }

        impl Default for DummyCircuit {
            fn default() -> Self {
                Self::new(0.into(), 0.into())
            }
        }

        impl Circuit<JubjubAffine> for DummyCircuit {
            type ConstraintSystem = Groth16<JubjubAffine>;
            fn synthesize(&self, composer: &mut Groth16<JubjubAffine>) -> Result<(), Error> {
                let x = composer.alloc_witness(self.x);
                let y = composer.alloc_witness(self.y);

                composer.append_edwards_expression(Expression::from(x), Expression::from(y));

                Ok(())
            }
        }

        let k = 9;
        let pp = Groth16Params::<TatePairing>::setup(k, OsRng);
        let x = BlsScalar::from_hex(
            "0x187d2619ff114316d237e86684fb6e3c6b15e9b924fa4e322764d3177508297a",
        )
        .unwrap();
        let y = BlsScalar::from_hex(
            "0x6230c613f1b460e026221be21cf4eabd5a8ea552db565cb18d3cabc39761eb9b",
        )
        .unwrap();

        let circuit = DummyCircuit::new(x, y);

        let (mut prover, verifier) = Groth16Key::<TatePairing, DummyCircuit>::compile(&pp)
            .expect("Failed to compile circuit");
        let proof = prover.create_proof(circuit).expect("Failed to prove");
        verifier
            .verify(&proof, &[])
            .expect("Failed to verify the proof");
    }

    #[test]
    fn r1cs_qap() {
        #[derive(Debug)]
        pub struct DummyCircuit {
            x: BlsScalar,
            o: BlsScalar,
        }

        impl DummyCircuit {
            pub fn new(x: BlsScalar, o: BlsScalar) -> Self {
                Self { x, o }
            }
        }

        impl Default for DummyCircuit {
            fn default() -> Self {
                Self::new(0.into(), 0.into())
            }
        }

        impl Circuit<JubjubAffine> for DummyCircuit {
            type ConstraintSystem = Groth16<JubjubAffine>;
            fn synthesize(&self, composer: &mut Groth16<JubjubAffine>) -> Result<(), Error> {
                let x = Expression::from(composer.alloc_instance(self.x));
                let o = composer.alloc_instance(self.o);

                let sym1 = composer.product(&x, &x);
                let y = composer.product(&sym1, &x);
                let sym2 = composer.sum(&y, &x);

                composer.assert_equal(
                    &(sym2 + Expression::from(BlsScalar::from(5))),
                    &Expression::from(o),
                );

                Ok(())
            }
        }

        let k = 9;
        let pp = Groth16Params::<TatePairing>::setup(k, OsRng);
        let x = BlsScalar::from(3);
        let o = BlsScalar::from(35);
        let circuit = DummyCircuit::new(x, o);

        let (mut prover, verifier) = Groth16Key::<TatePairing, DummyCircuit>::compile(&pp)
            .expect("Failed to compile circuit");
        let proof = prover.create_proof(circuit).expect("Failed to prove");
        verifier
            .verify(&proof, &[x, o])
            .expect("Failed to verify the proof");
    }
}
