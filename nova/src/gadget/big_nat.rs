use num_bigint::{BigInt, Sign};
use zkstd::circuit::prelude::{BinaryAssignment, FieldAssignment};
use zkstd::circuit::CircuitDriver;
use zkstd::common::PrimeField;
use zkstd::r1cs::R1cs;

pub(crate) const BN_LIMB_WIDTH: usize = 64;
pub(crate) const BN_N_LIMBS: usize = 4;

/// Convert a field element to a natural number
pub fn f_to_nat<F: PrimeField>(f: &F) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &f.to_raw_bytes())
}

/// Convert a natural number to a field element.
pub fn nat_to_f<F: PrimeField>(n: &BigInt) -> F {
    let mut bytes = n.to_signed_bytes_le();
    if bytes.len() > 64 {
        panic!("Length exceed the field size");
    };
    bytes.resize(64, 0);

    let mut res = [0; 64];
    res[0..64].copy_from_slice(&bytes);

    F::from_bytes_wide(&res)
}

/// Compute the limbs encoding a natural number.
/// The limbs are assumed to be based the `limb_width` power of 2.
pub fn nat_to_limbs<F: PrimeField>(nat: &BigInt, limb_width: usize, n_limbs: usize) -> Vec<F> {
    let mask = int_with_n_ones(limb_width);
    let mut nat = nat.clone();
    if nat.bits() as usize <= n_limbs * limb_width {
        (0..n_limbs)
            .map(|_| {
                let r = &nat & &mask;
                nat >>= limb_width as u32;
                nat_to_f(&r)
            })
            .collect()
    } else {
        panic!("Wrong amount of bits");
    }
}

fn int_with_n_ones(n: usize) -> BigInt {
    let mut m = BigInt::from(1);
    m <<= n as u32;
    m -= 1;
    m
}

/// Compute the natural number represented by an array of limbs.
/// The limbs are assumed to be based the `limb_width` power of 2.
pub fn limbs_to_nat<F: PrimeField, I: DoubleEndedIterator<Item = F>>(
    limbs: I,
    limb_width: usize,
) -> BigInt {
    limbs.rev().fold(BigInt::from(0), |mut acc, limb| {
        acc <<= limb_width as u32;
        acc += f_to_nat(&limb);
        acc
    })
}

#[derive(Clone)]
pub struct BigNatAssignment<F: PrimeField> {
    // LE
    limbs: Vec<FieldAssignment<F>>,
    params: BigNatParams,
}

#[derive(Clone, PartialEq, Eq)]
pub struct BigNatParams {
    pub min_bits: usize,
    pub max_word: BigInt,
    pub limb_width: usize,
    pub n_limbs: usize,
}

impl BigNatParams {
    pub fn new(limb_width: usize, n_limbs: usize) -> Self {
        BigNatParams {
            min_bits: 0,
            max_word: (BigInt::from(1) << limb_width as u32) - 1,
            n_limbs,
            limb_width,
        }
    }
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
            params: BigNatParams::new(limb_width, n_limbs),
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

    pub fn value<CS: CircuitDriver<Scalar = F>>(&self, cs: &R1cs<CS>) -> BigInt {
        limbs_to_nat(
            self.limbs.iter().map(|x| x.value(cs)),
            self.params.limb_width,
        )
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
            .rev()
            .map(|limb| FieldAssignment::to_bits::<CS>(cs, limb, self.params.limb_width))
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

        Self {
            limbs,
            params: a.params.clone(),
        }
    }

    // pub fn assert_well_formed() {}
    // pub fn enforce_limb_width_agreement(&self, other: &Self, location: &str) {}
    // pub fn red_mod(&self, modulus: &Self) -> Self {}
    // pub fn add(&self, other: &Self) -> Self {}    Maybe can merge with red_mod to create add_mod
    // pub fn mult_mod(&self, other: &Self, modulus: &Self) -> Self {}

    pub fn n_bits(&self) -> usize {
        BN_LIMB_WIDTH * (BN_N_LIMBS - 1) + self.params.max_word.bits() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::Bn254Driver;
    use bn_254::Fr;
    use rand_core::OsRng;
    use zkstd::common::Group;

    #[test]
    fn bignat_allocation_from_bigint() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let f = Fr::random(OsRng);
        let num = f_to_nat(&f);
        let num_assignment =
            BigNatAssignment::witness_from_big_int(&mut cs, num.clone(), BN_LIMB_WIDTH, BN_N_LIMBS);
        assert_eq!(num, num_assignment.value(&cs));
        assert!(cs.is_sat());
    }

    #[test]
    fn bignat_allocation_from_field_assignment() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let f = Fr::from(1 << 63) * Fr::from(1 << 3) - Fr::one();
        let num = f_to_nat(&f);
        let f_assignment = FieldAssignment::witness(&mut cs, f);
        let num_assignment = BigNatAssignment::witness_from_field_assignment(
            &mut cs,
            &f_assignment,
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );

        assert_eq!(num, num_assignment.value(&cs));
        assert!(cs.is_sat());
    }
}
