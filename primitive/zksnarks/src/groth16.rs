#![allow(unused_variables)]
mod constraint;
mod key;
mod matrix;
mod params;
mod prover;
mod verifier;
mod wire;

use crate::bit_iterator::BitIterator8;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;

pub(crate) mod curves;
pub(crate) mod error;
pub use prover::Prover;
pub use verifier::Verifier;

use core::ops::{Index, Neg};
use jub_jub::compute_windowed_naf;
use zkstd::common::{
    vec, FftField, Group, PrimeField, Ring, TwistedEdwardsAffine, TwistedEdwardsCurve,
    TwistedEdwardsExtended, Vec,
};

use constraint::R1csStruct;
use curves::EdwardsExpression;
use matrix::{Element, SparseRow};
use wire::Wire;

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
            instance: [Element::one()].to_vec(),
            witness: vec![],
        }
    }

    fn m(&self) -> usize {
        self.constraints.m()
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

impl<C: TwistedEdwardsAffine> Index<Wire> for Groth16<C> {
    type Output = C::Range;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Instance(i) => &self.instance[i].1,
            Wire::Witness(i) => &self.witness[i].1,
        }
    }
}

impl<C: TwistedEdwardsAffine> Groth16<C> {
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

    /// Appends a point in affine form as [`WitnessPoint`]
    pub fn append_point<A: Into<C>>(&mut self, affine: A) -> EdwardsExpression<C> {
        let affine = affine.into();

        let x = self.alloc_witness(affine.get_x());
        let y = self.alloc_witness(affine.get_y());

        self.append_edwards_expression(SparseRow::from(x), SparseRow::from(y))
    }

    pub fn append_edwards_expression(
        &mut self,
        x: SparseRow<C::Range>,
        y: SparseRow<C::Range>,
    ) -> EdwardsExpression<C> {
        let x_squared = self.product(&x, &x);
        let y_squared = self.product(&y, &y);
        let x_squared_y_squared = self.product(&x_squared, &y_squared);

        self.assert_equal(
            &y_squared,
            &(SparseRow::one() + x_squared_y_squared * C::PARAM_D + &x_squared),
        );

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Adds two points on an `EdwardsCurve` using the standard algorithm for Twisted Edwards
    /// Curves.
    pub fn add_points(
        &mut self,
        a: &EdwardsExpression<C>,
        b: &EdwardsExpression<C>,
    ) -> EdwardsExpression<C> {
        // In order to verify that two points were correctly added
        // without going over a degree 4 polynomial, we will need
        // x_1, y_1, x_2, y_2
        // x_3, y_3, x_1 * y_2

        let x_1 = a.x.as_constant().unwrap();
        let y_1 = a.y.as_constant().unwrap();
        let x_2 = b.x.as_constant().unwrap();
        let y_2 = b.y.as_constant().unwrap();

        let p1 = C::from_raw_unchecked(x_1, y_1);
        let p2 = C::from_raw_unchecked(x_2, y_2);

        let point = C::from(p1 + p2);

        let x_3 = point.get_x();
        let y_3 = point.get_y();

        let x1_y2 = x_1 * y_2;

        let x_1_y_2 = self.alloc_witness(x1_y2);
        let x_3 = self.alloc_witness(x_3);
        let y_3 = self.alloc_witness(y_3);

        // // Add the rest of the prepared points into the composer
        // let constraint = Constraint::default().a(x_1).b(y_1).o(x_2).d(y_2);
        // let constraint = Constraint::group_add_curve_addtion(constraint);
        //
        // self.append_custom_gate(constraint);
        //
        // let constraint = Constraint::default().a(x_3).b(y_3).d(x_1_y_2);
        //
        // self.append_custom_gate(constraint);

        EdwardsExpression::new_unsafe(SparseRow::from(x_3), SparseRow::from(y_3))
    }

    /// Performs scalar multiplication in constraints by first splitting up a scalar into
    /// a binary representation, and then performing the naive double-or-add algorithm. This
    /// implementation is generic for all groups.
    pub fn mul_point(
        &mut self,
        scalar: Wire,
        point: &EdwardsExpression<C>,
    ) -> EdwardsExpression<C> {
        let scalar_bits = self.component_decomposition::<252>(scalar);

        let mut result = EdwardsExpression::identity();

        for bit in scalar_bits.iter().rev() {
            result = self.add_points(&result, &result);

            let point_to_add = self.component_select_identity(*bit, point);
            result = self.add_points(&result, &point_to_add);
        }

        result
    }

    /// Evaluate `jubjub · Generator` as a [`WitnessPoint`]
    ///
    /// `generator` will be appended to the circuit description as constant
    ///
    /// Will error if `jubjub` doesn't fit `Fr`
    pub fn mul_generator<A: Into<C::Extended>>(
        &mut self,
        jubjub: Wire,
        generator: A,
    ) -> Result<EdwardsExpression<C>, Error> {
        let generator = generator.into();

        // the number of bits is truncated to the maximum possible. however, we
        // could slice off 3 bits from the top of wnaf since Fr price is
        // 252 bits. Alternatively, we could move to base4 and halve the
        // number of gates considering that the product of wnaf adjacent
        // entries is zero.
        let bits: usize = 256;

        // compute 2^iG
        let mut wnaf_point_multiples: Vec<C> = {
            let mut multiples = vec![C::Extended::ADDITIVE_IDENTITY; bits];

            multiples[0] = generator;

            for i in 1..bits {
                multiples[i] = multiples[i - 1].double();
            }

            multiples
                .iter()
                .map(|point| C::from(*point))
                .collect::<Vec<_>>()
        };

        wnaf_point_multiples.reverse();

        // we should error instead of producing invalid proofs - otherwise this
        // can easily become an attack vector to either shutdown prover
        // services or create malicious statements
        let scalar = self[jubjub];

        let width = 2;
        let wnaf_entries = compute_windowed_naf::<C::Range>(scalar, width);

        debug_assert_eq!(wnaf_entries.len(), bits);

        // initialize the accumulators
        let mut scalar_acc = vec![C::Range::zero()];
        let mut point_acc = vec![C::from(C::Extended::ADDITIVE_IDENTITY)];

        // auxillary point to help with checks on the backend
        let two = C::Range::from(2u64);
        let xy_alphas: Vec<_> = wnaf_entries
            .iter()
            .rev()
            .enumerate()
            .map(|(i, entry)| {
                let (scalar_to_add, point_to_add) = match entry {
                    0 => (C::Range::zero(), C::Extended::ADDITIVE_IDENTITY),
                    -1 => (
                        C::Range::one().neg(),
                        -(wnaf_point_multiples[i]).to_extended(),
                    ),
                    1 => (C::Range::one(), (wnaf_point_multiples[i]).to_extended()),
                    _ => return Err(Error::UnsupportedWNAF2k),
                };

                let prev_accumulator = two * scalar_acc[i];
                let scalar = prev_accumulator + scalar_to_add;
                scalar_acc.push(scalar);

                let point = point_acc[i] + point_to_add;
                point_acc.push(C::from(point));

                let point_to_add: C = point_to_add.into();

                let x_alpha = point_to_add.get_x();
                let y_alpha = point_to_add.get_y();

                Ok(x_alpha * y_alpha)
            })
            .collect::<Result<_, Error>>()?;

        for i in 0..bits {
            let acc_x = self.alloc_witness(point_acc[i].get_x());
            let acc_y = self.alloc_witness(point_acc[i].get_y());
            let accumulated_bit = self.alloc_witness(scalar_acc[i]);

            // the point accumulator must start from identity and its scalar
            // from zero
            if i == 0 {
                self.assert_equal(&SparseRow::from(acc_x), &SparseRow::from(C::Range::zero()));
                self.assert_equal(&SparseRow::from(acc_y), &SparseRow::from(Wire::ONE));
                self.assert_equal(
                    &SparseRow::from(accumulated_bit),
                    &SparseRow::from(C::Range::zero()),
                );
            }

            let x_beta = wnaf_point_multiples[i].get_x();
            let y_beta = wnaf_point_multiples[i].get_y();

            let xy_alpha = self.alloc_witness(xy_alphas[i]);
            let xy_beta = x_beta * y_beta;

            // let wnaf_round = WnafRound::<PrivateWire, C::Range> {
            //     acc_x,
            //     acc_y,
            //     accumulated_bit,
            //     xy_alpha,
            //     x_beta,
            //     y_beta,
            //     xy_beta,
            // };

            // let constraint = Constraint::group_add_curve_scalar(Constraint::default())
            //     .left(wnaf_round.x_beta)
            //     .right(wnaf_round.y_beta)
            //     .constant(wnaf_round.xy_beta)
            //     .a(wnaf_round.acc_x)
            //     .b(wnaf_round.acc_y)
            //     .o(wnaf_round.xy_alpha)
            //     .d(wnaf_round.accumulated_bit);
            //
            // self.append_custom_gate(constraint)
        }

        // last gate isn't activated for ecc
        let acc_x = self.alloc_witness(point_acc[bits].get_x());
        let acc_y = self.alloc_witness(point_acc[bits].get_y());

        // FIXME this implementation presents a plethora of vulnerabilities and
        // requires reworking
        //
        // we are accepting any scalar argument and trusting it to be the
        // expected input. it happens to be correct in this
        // implementation, but can be exploited by malicious provers who
        // might just input anything here
        let last_accumulated_bit = self.alloc_witness(scalar_acc[bits]);

        // // FIXME the gate isn't checking anything. maybe remove?
        // let constraint = Constraint::default()
        //     .a(acc_x)
        //     .b(acc_y)
        //     .d(last_accumulated_bit);
        // self.append_gate(constraint);

        // constrain the last element in the accumulator to be equal to the
        // input jubjub scalar
        self.assert_equal(
            &SparseRow::from(last_accumulated_bit),
            &SparseRow::from(jubjub),
        );

        Ok(EdwardsExpression::new_unsafe(
            SparseRow::from(acc_x),
            SparseRow::from(acc_y),
        ))
    }

    /// Conditionally selects identity as [`WitnessPoint`] based on an input
    /// bit.
    ///
    /// bit == 1 => a,
    /// bit == 0 => identity,
    ///
    /// `bit` is expected to be constrained by
    /// [`Composer::component_boolean`]
    pub fn component_select_identity(
        &mut self,
        bit: Wire,
        a: &EdwardsExpression<C>,
    ) -> EdwardsExpression<C> {
        let x = SparseRow::from(self.component_select_zero(bit, &a.x));
        let y = SparseRow::from(self.component_select_one(bit, &a.y));

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Conditionally selects a [`PrivateWire`] based on an input bit.
    ///
    /// bit == 1 => value,
    /// bit == 0 => 0,
    ///
    /// `bit` is expected to be constrained by
    /// [`Composer::component_boolean`]
    pub fn component_select_zero(&mut self, bit: Wire, value: &SparseRow<C::Range>) -> Wire {
        let mul = self.product(&SparseRow::from(bit), value);
        self.alloc_witness(mul.evaluate(&self.instance, &self.witness))
    }

    /// Conditionally selects a [`PrivateWire`] based on an input bit.
    ///
    /// bit == 1 => value,
    /// bit == 0 => 1,
    ///
    /// `bit` is expected to be constrained by
    /// [`Composer::component_boolean`]
    pub fn component_select_one(&mut self, bit: Wire, value: &SparseRow<C::Range>) -> Wire {
        let b = SparseRow::from(bit);
        let mul = self.product(&b, value);

        let f_x = SparseRow::from(C::Range::one()) - &b + mul;
        self.assert_product(&-b, &-value, &f_x);
        self.alloc_witness(f_x.evaluate(&self.instance, &self.witness))
    }

    /// Decomposes `scalar` into an array truncated to `N` bits (max 256).
    ///
    /// Asserts the reconstruction of the bits to be equal to `scalar`.
    ///
    /// Consume `2 · N + 1` gates
    pub fn component_decomposition<const N: usize>(&mut self, scalar: Wire) -> [Wire; N] {
        // Static assertion
        assert!(0 < N && N <= 256);

        let mut decomposition = [Wire::ONE; N];
        let scalar = SparseRow::<C::Range>::from(scalar);

        let acc = Wire::ONE;
        let acc = scalar
            .as_constant()
            .expect("Failed to get wire value")
            .to_bits()
            .iter()
            .rev()
            .enumerate()
            .zip(decomposition.iter_mut())
            .fold(acc, |acc, ((i, w), d)| {
                *d = self.alloc_witness(C::Range::from(*w as u64));

                let left = self.product(
                    &SparseRow::from(*d),
                    &SparseRow::from(C::Range::pow_of_2(i as u64)),
                );
                let right = SparseRow::from(acc);

                self.assert_equal(&left, &right);

                self.alloc_witness((left + right).evaluate(&self.instance, &self.witness))
            });

        self.assert_equal(&SparseRow::from(acc), &scalar);

        decomposition
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

    /// Asserts `a == b` by appending two gates
    pub fn assert_equal_point(&mut self, a: &EdwardsExpression<C>, b: &EdwardsExpression<C>) {
        self.assert_equal(&a.x, &b.x);
        self.assert_equal(&a.y, &b.y);
    }

    /// Asserts `point == public`.
    ///
    /// Will add `public` affine coordinates `(x,y)` as public inputs
    pub fn assert_equal_public_point<A: Into<C>>(
        &mut self,
        point: &EdwardsExpression<C>,
        public: A,
    ) {
        let public = public.into();

        self.assert_equal(&point.x, &SparseRow::from(public.get_x()));
        self.assert_equal(&point.y, &SparseRow::from(public.get_y()));
    }

    /// Adds a range-constraint gate that checks and constrains a
    /// [`PrivateWire`] to be inside of the range \[0,num_bits\].
    ///
    /// This function adds `num_bits/4` gates to the circuit description in
    /// order to add the range constraint.
    ///
    ///# Panics
    /// This function will panic if the num_bits specified is not even, ie.
    /// `num_bits % 2 != 0`.
    pub fn range(&mut self, witness: Wire, num_bits: usize) {
        // convert witness to bit representation and reverse
        let bits = self[witness];
        let bit_iter = BitIterator8::new(bits.to_raw_bytes());
        let mut bits: Vec<_> = bit_iter.collect();
        bits.reverse();

        // considering this is a width-4 program, one gate will contain 4
        // accumulators. each accumulator proves that a single quad is a
        // base-4 digit. accumulators are bijective to quads, and these
        // are 2-bits each. given that, one gate accumulates 8 bits.
        let mut num_gates = num_bits >> 3;

        // given each gate accumulates 8 bits, its count must be padded
        if num_bits % 8 != 0 {
            num_gates += 1;
        }

        // a gate holds 4 quads
        let num_quads = num_gates * 4;

        // the wires are left-padded with the difference between the quads count
        // and the bits argument
        let pad = 1 + (((num_quads << 1) - num_bits) >> 1);

        // last gate is reserved for either the genesis quad or the padding
        let used_gates = num_gates + 1;

        // let base = Constraint::<C::Range>::default();
        // let base = Constraint::range(base);
        // let mut constraints = vec![base; used_gates];
        //
        // // We collect the set of accumulators to return back to the user
        // // and keep a running count of the current accumulator
        let accumulators: Vec<Wire> = Vec::new();
        // let mut accumulator = C::Range::zero();
        // let four = C::Range::from(4);
        //
        // for i in pad..=num_quads {
        //     // convert each pair of bits to quads
        //     let bit_index = (num_quads - i) << 1;
        //     let q_0 = bits[bit_index] as u64;
        //     let q_1 = bits[bit_index + 1] as u64;
        //     let quad = q_0 + (2 * q_1);
        //
        //     accumulator = four * accumulator;
        //     accumulator += C::Range::from(quad);
        //
        //     let accumulator_var = self.alloc_witness(accumulator);
        //
        //     accumulators.push(accumulator_var);
        //
        //     let idx = i / 4;
        //     match i % 4 {
        //         0 => {
        //             constraints[idx].w_d = accumulator_var;
        //         }
        //         1 => {
        //             constraints[idx].w_o = accumulator_var;
        //         }
        //         2 => {
        //             constraints[idx].w_b = accumulator_var;
        //         }
        //         3 => {
        //             constraints[idx].w_a = accumulator_var;
        //         }
        //         _ => unreachable!(),
        //     };
        // }
        //
        // // last constraint is zeroed as it is reserved for the genesis quad or
        // // padding
        // if let Some(c) = constraints.last_mut() {
        //     *c = Constraint::default()
        // }
        //
        // // the accumulators count is a function to the number of quads. hence,
        // // this optional gate will not cause different circuits depending on the
        // // witness because this computation is bound to the constant bits count
        // // alone.
        // if let Some(accumulator) = accumulators.last() {
        //     if let Some(c) = constraints.last_mut() {
        //         c.w_d = *accumulator
        //     }
        // }
        //
        // constraints
        //     .into_iter()
        //     .for_each(|c| self.append_custom_gate(c));

        // the accumulators count is a function to the number of quads. hence,
        // this optional gate will not cause different circuits depending on the
        // witness because this computation is bound to the constant bits count
        // alone.
        if let Some(accumulator) = accumulators.last() {
            self.assert_equal(&SparseRow::from(accumulator), &SparseRow::from(witness));
        }
    }

    /// The product of two `SparseRow`s `x` and `y`, i.e. `x * y`.
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

    /// The product of two `SparseRow`s `x` and `y`, i.e. `x * y`.
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

        let (mut prover, verifier) =
            Groth16Key::<TatePairing, JubjubAffine, DummyCircuit>::compile(&pp)
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

        let (mut prover, verifier) =
            Groth16Key::<TatePairing, JubjubAffine, DummyCircuit>::compile(&pp)
                .expect("Failed to compile circuit");
        let proof = prover
            .create_proof(&mut OsRng, circuit)
            .expect("Failed to prove");
        verifier
            .verify(&proof, &[x, o])
            .expect("Failed to verify the proof");
    }
}
