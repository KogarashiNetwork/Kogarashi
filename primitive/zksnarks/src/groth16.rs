#![allow(dead_code)]
mod constraint;
mod key;
mod matrix;
mod params;
mod prover;
mod verifier;

pub(crate) mod curves;
pub(crate) mod error;
pub mod wire;

use crate::constraint_system::ConstraintSystem;

use constraint::R1csStruct;
use curves::EdwardsExpression;
use matrix::{Element, SparseRow};
use wire::Wire;
use zkstd::common::{vec, Group, TwistedEdwardsAffine, Vec};

#[derive(Debug)]
pub struct Groth16<C: TwistedEdwardsAffine> {
    constraints: R1csStruct<C::Range>,
    pub(crate) instance: Vec<Element<C::Range>>,
    pub(crate) witness: Vec<Element<C::Range>>,
}

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> for Groth16<C> {
    type Wire = Wire;
    type Constraints = R1csStruct<C::Range>;

    fn initialize() -> Self {
        Self {
            constraints: R1csStruct::default(),
            instance: [Element::one()].into_iter().collect(),
            witness: vec![],
        }
    }

    fn m(&self) -> usize {
        self.constraints().m()
    }

    fn constraints(&self) -> Self::Constraints {
        self.constraints.clone()
    }

    fn alloc_instance(&mut self, instance: C::Range) -> Wire {
        let wire = self.public_wire();
        self.instance.push(Element(wire, instance));
        wire
    }

    fn alloc_witness(&mut self, witness: C::Range) -> Wire {
        let wire = self.private_wire();
        self.witness.push(Element(wire, witness));
        wire
    }
}

impl<C: TwistedEdwardsAffine> Groth16<C> {
    #[allow(clippy::type_complexity)]
    fn inputs_iter(
        &self,
    ) -> (
        (
            Vec<Vec<(C::Range, usize)>>,
            Vec<Vec<(C::Range, usize)>>,
            Vec<Vec<(C::Range, usize)>>,
        ),
        (
            Vec<Vec<(C::Range, usize)>>,
            Vec<Vec<(C::Range, usize)>>,
            Vec<Vec<(C::Range, usize)>>,
        ),
    ) {
        let mut a_instance = vec![vec![]; self.instance_len()];
        let mut b_instance = vec![vec![]; self.instance_len()];
        let mut c_instance = vec![vec![]; self.instance_len()];
        let mut a_witness = vec![vec![]; self.witness_len()];
        let mut b_witness = vec![vec![]; self.witness_len()];
        let mut c_witness = vec![vec![]; self.witness_len()];
        for (i, ((a, b), c)) in self
            .constraints
            .a
            .0
            .iter()
            .zip(self.constraints.b.0.iter())
            .zip(self.constraints.c.0.iter())
            .enumerate()
        {
            a.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => a_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => a_witness[*k].push((*coeff, i)),
                });
            b.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => b_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => b_witness[*k].push((*coeff, i)),
                });
            c.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => c_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => c_witness[*k].push((*coeff, i)),
                });
        }

        (
            (a_instance, b_instance, c_instance),
            (a_witness, b_witness, c_witness),
        )
    }

    fn eval_constraints(&mut self) -> (Vec<C::Range>, Vec<C::Range>, Vec<C::Range>) {
        self.instance.sort();
        self.witness.sort();
        self.constraints.evaluate(&self.instance, &self.witness)
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
        Wire::Instance(index)
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.witness.len();
        Wire::Witness(index)
    }

    pub fn append_edwards_expression(
        &mut self,
        x: SparseRow<C::Range>,
        y: SparseRow<C::Range>,
    ) -> EdwardsExpression<C::Range, C> {
        let x_squared = self.product(&x, &x);
        let y_squared = self.product(&y, &y);
        let x_squared_y_squared = self.product(&x_squared, &y_squared);

        self.assert_equal(
            &y_squared,
            &(SparseRow::one() + x_squared_y_squared * C::PARAM_D + &x_squared),
        );

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Assert that x * y = z;
    pub fn assert_product(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
        z: &SparseRow<C::Range>,
    ) {
        self.constraints.append(x.clone(), y.clone(), z.clone());
    }

    // Assert that x + y = z;
    pub fn assert_sum(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
        z: &SparseRow<C::Range>,
    ) {
        self.constraints
            .append(x + y, SparseRow::from(Wire::ONE), z.clone());
    }

    /// Assert that x == y.
    pub fn assert_equal(&mut self, x: &SparseRow<C::Range>, y: &SparseRow<C::Range>) {
        self.assert_product(x, &SparseRow::one(), y);
    }

    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn product(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
    ) -> SparseRow<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value =
            x.evaluate(&self.instance, &self.witness) * y.evaluate(&self.instance, &self.witness);
        let product = self.alloc_witness(product_value);
        let product_exp = SparseRow::from(product);
        self.assert_product(x, y, &product_exp);

        product_exp
    }

    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn sum(&mut self, x: &SparseRow<C::Range>, y: &SparseRow<C::Range>) -> SparseRow<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let sum_value =
            x.evaluate(&self.instance, &self.witness) + y.evaluate(&self.instance, &self.witness);
        let sum = self.alloc_witness(sum_value);
        let sum_exp = SparseRow::from(sum);
        self.assert_sum(x, y, &sum_exp);
        sum_exp
    }

    /// Returns `1 / x`, assuming `x` is non-zero. If `x` is zero, the gadget will not be
    /// satisfiable.
    pub fn inverse(&mut self, x: &SparseRow<C::Range>) -> SparseRow<C::Range> {
        let x_value = x.evaluate(&self.instance, &self.witness);
        let inverse_value = x_value.invert().expect("Can't find an inverse element");
        let x_inv = self.alloc_witness(inverse_value);

        let x_inv_expression = SparseRow::from(x_inv);
        self.assert_product(x, &x_inv_expression, &SparseRow::one());

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
    use jub_jub::JubjubAffine;
    use matrix::SparseRow;
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

                composer.append_edwards_expression(SparseRow::from(x), SparseRow::from(y));

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
        let proof = prover
            .create_proof(&mut OsRng, circuit)
            .expect("Failed to prove");
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
                let x = SparseRow::from(composer.alloc_instance(self.x));
                let o = composer.alloc_instance(self.o);

                let sym1 = composer.product(&x, &x);
                let y = composer.product(&sym1, &x);
                let sym2 = composer.sum(&y, &x);

                composer.assert_equal(
                    &(sym2 + SparseRow::from(BlsScalar::from(5))),
                    &SparseRow::from(o),
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
        let proof = prover
            .create_proof(&mut OsRng, circuit)
            .expect("Failed to prove");
        verifier
            .verify(&proof, &[x, o])
            .expect("Failed to verify the proof");
    }
}
