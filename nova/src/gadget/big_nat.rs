use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
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
    pub fn witness_from_field_assignment<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
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

    pub fn from_poly(poly: Polynomial<F>, limb_width: usize, max_word: BigInt) -> Self {
        Self {
            params: BigNatParams {
                min_bits: 0,
                max_word,
                n_limbs: poly.coefficients.len(),
                limb_width,
            },
            limbs: poly.coefficients,
        }
    }

    pub fn enforce_well_formed<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>) {
        for limb in &self.limbs {
            let limb_bits = FieldAssignment::to_bits(cs, limb, 256);
            FieldAssignment::range_check_bits(cs, &limb_bits, self.params.limb_width as u64);
        }
    }

    pub fn value<C: CircuitDriver<Scalar = F>>(&self, cs: &R1cs<C>) -> BigInt {
        limbs_to_nat(
            self.limbs.iter().map(|x| x.value(cs)),
            self.params.limb_width,
        )
    }

    pub fn as_limbs(&self) -> Vec<FieldAssignment<F>> {
        self.limbs.clone()
    }

    /// Break `self` up into a bit-vector.
    pub fn decompose<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
    ) -> Vec<BinaryAssignment> {
        let limb_values_split = self.limbs.iter().map(|limb| limb.value(cs));
        let bitvectors: Vec<Vec<BinaryAssignment>> = self
            .limbs
            .iter()
            .rev()
            .map(|limb| FieldAssignment::to_bits::<C>(cs, limb, self.params.limb_width))
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
        assert_eq!(a.limbs.len(), b.limbs.len());
        let mut limbs = vec![];
        for (a, b) in a.limbs.iter().zip(b.limbs.iter()) {
            limbs.push(FieldAssignment::conditional_select(cs, a, b, condition));
        }

        Self {
            limbs,
            params: a.params.clone(),
        }
    }

    pub fn enforce_equal_when_carried_regroup<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        other: &Self,
    ) {
        assert_eq!(self.params.limb_width, other.params.limb_width);
        let max_word = std::cmp::max(&self.params.max_word, &other.params.max_word);
        let carry_bits =
            (((max_word.to_f64().unwrap() * 2.0).log2() - self.params.limb_width as f64).ceil()
                + 0.1) as usize;
        let limbs_per_group = ((C::NUM_BITS - 1) as usize - carry_bits) / self.params.limb_width;
        let self_grouped = self.group_limbs(cs, limbs_per_group);
        let other_grouped = other.group_limbs(cs, limbs_per_group);
        self_grouped.enforce_equal_when_carried(cs, &other_grouped)
    }

    pub fn enforce_equal_when_carried<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        other: &Self,
    ) {
        assert_eq!(self.params.limb_width, other.params.limb_width);

        let n = std::cmp::min(self.limbs.len(), other.limbs.len());
        let target_base = BigInt::from(1u8) << self.params.limb_width as u32;
        let mut accumulated_extra = BigInt::from(0usize);
        let max_word = std::cmp::max(&self.params.max_word, &other.params.max_word);
        let carry_bits =
            (((max_word.to_f64().unwrap() * 2.0).log2() - self.params.limb_width as f64).ceil()
                + 0.1) as usize;
        let mut carry_in = FieldAssignment::constant(&F::zero());

        for i in 0..n {
            let carry = FieldAssignment::witness(
                cs,
                nat_to_f(
                    &((f_to_nat(&self.limbs[i].value(cs))
                        + f_to_nat(&carry_in.value(cs))
                        + max_word
                        - f_to_nat(&other.limbs[i].value(cs)))
                        / &target_base),
                ),
            );
            accumulated_extra += max_word;

            let target = FieldAssignment::mul(
                cs,
                &FieldAssignment::constant(&nat_to_f::<F>(&target_base)),
                &carry,
            );
            let carry_sum = &(&carry_in + &self.limbs[i]) - &other.limbs[i];
            let accumulated =
                FieldAssignment::constant(&nat_to_f::<F>(&(&accumulated_extra % &target_base)));
            let max_word = FieldAssignment::constant(&nat_to_f::<F>(max_word));

            FieldAssignment::enforce_eq_constant(
                cs,
                &(&(&(&carry_sum + &max_word) - &target) - &accumulated),
                &F::zero(),
            );

            accumulated_extra /= &target_base;
            if i < n - 1 {
                let carry_decomposition = FieldAssignment::to_bits(cs, &carry, 256);
                FieldAssignment::range_check_bits(cs, &carry_decomposition, carry_bits as u64);
            } else {
                FieldAssignment::enforce_eq_constant(
                    cs,
                    &(&carry - &FieldAssignment::constant(&nat_to_f(&accumulated_extra))),
                    &F::zero(),
                );
            }
            carry_in = carry;
        }

        for limb in self.limbs.iter().skip(n) {
            FieldAssignment::enforce_eq_constant(cs, limb, &F::zero());
        }

        for limb in other.limbs.iter().skip(n) {
            FieldAssignment::enforce_eq_constant(cs, limb, &F::zero());
        }
    }

    pub fn mult_mod<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        other: &Self,
        modulus: &Self,
    ) -> Self {
        assert_eq!(self.params.limb_width, other.params.limb_width);
        let limb_width = self.params.limb_width;
        let quotient_bits =
            (self.n_bits() + other.n_bits()).saturating_sub(modulus.params.min_bits);
        let quotient_limbs = quotient_bits.saturating_sub(1) / limb_width + 1;
        let quotient = BigNatAssignment::witness_from_big_int(
            cs,
            (self.value(cs) * other.value(cs)) / modulus.value(cs),
            self.params.limb_width,
            quotient_limbs,
        );
        quotient.enforce_well_formed(cs);
        let remainder = BigNatAssignment::witness_from_big_int(
            cs,
            (self.value(cs) * other.value(cs)) % modulus.value(cs),
            self.params.limb_width,
            modulus.limbs.len(),
        );
        remainder.enforce_well_formed(cs);
        let a_poly = Polynomial::from(self.clone());
        let b_poly = Polynomial::from(other.clone());
        let mod_poly = Polynomial::from(modulus.clone());
        let q_poly = Polynomial::from(quotient.clone());
        let r_poly = Polynomial::from(remainder.clone());

        // a * b
        let left = a_poly.mul(cs, &b_poly);
        let right_product = q_poly.mul(cs, &mod_poly);
        // q * m + r
        let right = right_product.add(cs, &r_poly);

        let left_max_word = {
            let mut x = BigInt::from(std::cmp::min(self.limbs.len(), other.limbs.len()));
            x *= &self.params.max_word;
            x *= &other.params.max_word;
            x
        };
        let right_max_word = {
            let mut x = BigInt::from(std::cmp::min(quotient.limbs.len(), modulus.limbs.len()));
            x *= &quotient.params.max_word;
            x *= &modulus.params.max_word;
            x += &remainder.params.max_word;
            x
        };

        let left_int = BigNatAssignment::from_poly(left, limb_width, left_max_word);
        let right_int = BigNatAssignment::from_poly(right, limb_width, right_max_word);
        left_int.enforce_equal_when_carried_regroup(cs, &right_int);
        remainder
    }

    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.params.limb_width, other.params.limb_width);
        let n_limbs = std::cmp::max(self.params.n_limbs, other.params.n_limbs);
        let max_word = &self.params.max_word + &other.params.max_word;
        let limbs: Vec<FieldAssignment<F>> = (0..n_limbs)
            .map(|i| match (self.limbs.get(i), other.limbs.get(i)) {
                (Some(a), Some(b)) => a + b,
                (Some(a), None) => a.clone(),
                (None, Some(b)) => b.clone(),
                (None, None) => unreachable!(),
            })
            .collect();
        Self {
            limbs,
            params: BigNatParams {
                min_bits: std::cmp::max(self.params.min_bits, other.params.min_bits),
                n_limbs,
                max_word,
                limb_width: self.params.limb_width,
            },
        }
    }

    pub fn red_mod<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>, modulus: &Self) -> Self {
        assert_eq!(self.params.limb_width, modulus.params.limb_width);
        let limb_width = self.params.limb_width;
        let quotient_bits = self.n_bits().saturating_sub(modulus.params.min_bits);
        let quotient_limbs = quotient_bits.saturating_sub(1) / limb_width + 1;
        let quotient = BigNatAssignment::witness_from_big_int(
            cs,
            self.value(cs) / modulus.value(cs),
            self.params.limb_width,
            quotient_limbs,
        );
        quotient.enforce_well_formed(cs);
        let remainder = BigNatAssignment::witness_from_big_int(
            cs,
            self.value(cs) % modulus.value(cs),
            self.params.limb_width,
            modulus.limbs.len(),
        );
        remainder.enforce_well_formed(cs);

        let mod_poly = Polynomial::from(modulus.clone());
        let q_poly = Polynomial::from(quotient.clone());
        let r_poly = Polynomial::from(remainder.clone());

        // q * m + r
        let right_product = q_poly.mul(cs, &mod_poly);
        let right = right_product.add(cs, &r_poly);

        let right_max_word = {
            let mut x = BigInt::from(std::cmp::min(quotient.limbs.len(), modulus.limbs.len()));
            x *= &quotient.params.max_word;
            x *= &modulus.params.max_word;
            x += &remainder.params.max_word;
            x
        };

        let right_int = BigNatAssignment::from_poly(right, limb_width, right_max_word);
        self.enforce_equal_when_carried_regroup(cs, &right_int);
        remainder
    }

    pub fn group_limbs<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        limbs_per_group: usize,
    ) -> Self {
        let n_groups = (self.limbs.len() - 1) / limbs_per_group + 1;
        let mut limbs = vec![FieldAssignment::constant(&F::zero()); n_groups];
        let mut shift = FieldAssignment::constant(&F::one());
        let limb_block = FieldAssignment::constant(&F::pow_of_2(self.params.limb_width as u64));
        for (i, limb) in self.limbs.iter().enumerate() {
            if i % limbs_per_group == 0 {
                shift = FieldAssignment::constant(&F::one());
            }

            limbs[i / limbs_per_group] =
                &limbs[i / limbs_per_group] + &FieldAssignment::mul(cs, &shift, limb);
            shift = FieldAssignment::mul(cs, &shift, &limb_block);
        }

        let max_word = (0..limbs_per_group).fold(BigInt::from(0u8), |mut acc, i| {
            acc.set_bit((i * self.params.limb_width) as u64, true);
            acc
        }) * &self.params.max_word;
        Self {
            params: BigNatParams {
                min_bits: self.params.min_bits,
                limb_width: self.params.limb_width * limbs_per_group,
                n_limbs: limbs.len(),
                max_word,
            },
            limbs,
        }
    }

    pub fn n_bits(&self) -> usize {
        BN_LIMB_WIDTH * (BN_N_LIMBS - 1) + self.params.max_word.bits() as usize
    }
}

pub struct Polynomial<F: PrimeField> {
    pub coefficients: Vec<FieldAssignment<F>>,
}

impl<F: PrimeField> Polynomial<F> {
    pub fn mul<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>, other: &Self) -> Self {
        let n_product_coeffs = self.coefficients.len() + other.coefficients.len() - 1;

        let mut product: Vec<FieldAssignment<F>> =
            std::iter::repeat_with(|| FieldAssignment::constant(&F::zero()))
                .take(n_product_coeffs)
                .collect();
        for (i, a) in self.coefficients.iter().enumerate() {
            for (j, b) in other.coefficients.iter().enumerate() {
                let mul = FieldAssignment::mul(cs, a, b);
                product[i + j] = &product[i + j] + &mul;
            }
        }

        let one = F::one();
        let mut x = F::zero();
        for _ in 0..n_product_coeffs {
            x += one;
            let mut i = F::one();
            let a =
                self.coefficients
                    .iter()
                    .fold(FieldAssignment::constant(&F::zero()), |acc, c| {
                        let r = &acc + &FieldAssignment::mul(cs, c, &FieldAssignment::constant(&i));
                        i *= x;
                        r
                    });
            let mut i = F::one();
            let b =
                other
                    .coefficients
                    .iter()
                    .fold(FieldAssignment::constant(&F::zero()), |acc, c| {
                        let r = &acc + &FieldAssignment::mul(cs, c, &FieldAssignment::constant(&i));
                        i *= x;
                        r
                    });
            let mut i = F::one();
            let c = product
                .iter()
                .fold(FieldAssignment::constant(&F::zero()), |acc, c| {
                    let r = &acc + &FieldAssignment::mul(cs, c, &FieldAssignment::constant(&i));
                    i *= x;
                    r
                });
            let ab = FieldAssignment::mul(cs, &a, &b);
            FieldAssignment::enforce_eq(cs, &ab, &c);
        }

        Self {
            coefficients: product,
        }
    }
    pub fn add<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>, other: &Self) -> Self {
        let n_coeffs = std::cmp::max(self.coefficients.len(), other.coefficients.len());
        let sum = (0..n_coeffs)
            .map(|i| {
                let mut s = FieldAssignment::constant(&F::zero());
                if i < self.coefficients.len() {
                    s = &s + &self.coefficients[i];
                }
                if i < other.coefficients.len() {
                    s = &s + &other.coefficients[i];
                }
                s
            })
            .collect();

        Polynomial { coefficients: sum }
    }
}

impl<F: PrimeField> From<BigNatAssignment<F>> for Polynomial<F> {
    fn from(bn: BigNatAssignment<F>) -> Polynomial<F> {
        Polynomial {
            coefficients: bn.limbs,
        }
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

    #[test]
    fn bignat_add() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let modulus = f_to_nat(&(Fr::MODULUS - Fr::one()));
        let num1_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            modulus.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let num2_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            modulus.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );

        let sum = num1_assignment.add(&num2_assignment);
        let sum_native = modulus.clone() + modulus;
        assert_eq!(sum_native, sum.value(&cs));
        assert!(cs.is_sat());
    }

    #[test]
    fn bignat_red_mod() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let value = BigInt::from(42);
        let modulus = BigInt::from(5);
        let value_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            value.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let modulus_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            modulus.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );

        let remainder = value_assignment.red_mod(&mut cs, &modulus_assignment);
        let remainder_native = value % modulus;
        assert_eq!(remainder_native, remainder.value(&cs));
        assert!(cs.is_sat());
    }

    #[test]
    fn bignat_mult_mod() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let num1 = BigInt::from(42);
        let num2 = BigInt::from(1337);
        let modulus = BigInt::from(322);
        let num1_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            num1.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let num2_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            num2.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let modulus_assignment = BigNatAssignment::witness_from_big_int(
            &mut cs,
            modulus.clone(),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );

        let mult_mod = num1_assignment.mult_mod(&mut cs, &num2_assignment, &modulus_assignment);
        let mult_mod_native = (num1 * num2) % modulus;
        assert_eq!(mult_mod_native, mult_mod.value(&cs));
        assert!(cs.is_sat());
    }
}
