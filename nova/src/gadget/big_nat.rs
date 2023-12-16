use crate::driver::nat_to_limbs;
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
    pub fn witness<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, num: BigInt) -> Self {
        let limb_values = nat_to_limbs::<F>(&num, BN_LIMB_WIDTH, BN_N_LIMBS);
        let mut limbs = vec![FieldAssignment::constant(&F::zero()); BN_N_LIMBS];
        for (i, limb) in limb_values.iter().enumerate() {
            limbs[i] = FieldAssignment::witness(cs, *limb);
        }
        dbg!(limb_values);

        Self {
            limbs,
            max_word: num,
        }
    }

    pub fn conditional_select<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        a: &Self,
        b: &Self,
        condition: &BinaryAssignment,
    ) -> BigNatAssignment<F> {
        let mut limbs = vec![FieldAssignment::constant(&F::zero()); BN_N_LIMBS];
        for i in 0..BN_N_LIMBS {
            limbs[i] = FieldAssignment::conditional_select(cs, &a.limbs[i], &b.limbs[i], condition);
        }

        let max_word = if cs[*condition.inner()] == F::one() {
            a.max_word.clone()
        } else {
            b.max_word.clone()
        };

        Self { limbs, max_word }
    }

    pub fn n_bits(&self) -> usize {
        BN_LIMB_WIDTH * (BN_N_LIMBS - 1) + self.max_word.bits() as usize
    }
}
