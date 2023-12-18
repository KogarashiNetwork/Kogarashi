use crate::driver::{f_to_nat, nat_to_limbs};
use num_bigint::BigInt;
use zkstd::circuit::prelude::{BinaryAssignment, FieldAssignment};
use zkstd::circuit::CircuitDriver;
use zkstd::common::PrimeField;
use zkstd::r1cs::R1cs;

pub(crate) const BN_LIMB_WIDTH: usize = 64;
pub(crate) const BN_N_LIMBS: usize = 4;

#[derive(Clone)]
pub struct BigNatAssignment<F: PrimeField> {
    limbs: Vec<FieldAssignment<F>>,
    max_word: BigInt,
}

impl<F: PrimeField> BigNatAssignment<F> {
    pub fn witness_from_big_int<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        num: BigInt,
        limb_width: usize,
        n_limbs: usize,
    ) -> Self {
        let limb_values = nat_to_limbs::<F>(&num, limb_width, n_limbs);
        let mut limbs = vec![FieldAssignment::constant(&F::zero()); n_limbs];
        for (i, limb) in limb_values.iter().enumerate() {
            limbs[i] = FieldAssignment::witness(cs, *limb);
        }

        Self {
            limbs,
            max_word: num,
        }
    }

    /// Allocates a `BigNat` in the circuit with `n_limbs` limbs of width `limb_width` each.
    /// The `max_word` is guaranteed to be `(2 << limb_width) - 1`.
    /// The value is provided by an allocated number
    pub fn witness_from_field_assignment<CS: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<CS>,
        f: &FieldAssignment<F>,
        limb_width: usize,
        n_limbs: usize,
    ) -> Self {
        let big_nat = Self::witness_from_big_int(cs, f_to_nat(&f.value(cs)), limb_width, n_limbs);

        // check if bignat equals n
        // (1) decompose `bignat` into a bitvector `bv`
        let bv = big_nat.decompose(cs);
        // (2) recompose bits and check if it equals n
        FieldAssignment::enforce_eq_bits(cs, f, &bv);

        big_nat
    }

    pub fn as_limbs(&self) -> Vec<FieldAssignment<F>> {
        self.limbs.clone()
    }

    /// Break `self` up into a bit-vector.
    pub fn decompose<CS: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<CS>,
    ) -> Vec<BinaryAssignment> {
        let limb_values_split = self.limbs.iter().map(|limb| limb.value(cs));
        let bitvectors: Vec<Vec<BinaryAssignment>> = self
            .limbs
            .iter()
            .map(|limb| FieldAssignment::to_bits(cs, limb))
            .collect();
        let mut bits = Vec::new();

        for bv in bitvectors {
            bits.extend(bv);
        }
        bits
    }

    pub fn conditional_select<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        a: &Self,
        b: &Self,
        condition: &BinaryAssignment,
    ) -> BigNatAssignment<F> {
        let mut limbs = vec![FieldAssignment::constant(&F::zero()); BN_N_LIMBS];
        for (i, limb) in limbs.iter_mut().enumerate().take(BN_N_LIMBS) {
            *limb = FieldAssignment::conditional_select(cs, &a.limbs[i], &b.limbs[i], condition);
        }

        let max_word = if cs[*condition.inner()] == F::one() {
            a.max_word.clone()
        } else {
            b.max_word.clone()
        };

        Self { limbs, max_word }
    }

    // pub fn assert_well_formed() {}
    // pub fn enforce_limb_width_agreement(&self, other: &Self, location: &str) {}
    // pub fn red_mod(&self, modulus: &Self) -> Self {}
    // pub fn add(&self, other: &Self) -> Self {}    Maybe can merge with red_mod to create add_mod
    // pub fn mult_mod(&self, other: &Self, modulus: &Self) -> Self {}

    pub fn n_bits(&self) -> usize {
        BN_LIMB_WIDTH * (BN_N_LIMBS - 1) + self.max_word.bits() as usize
    }
}
