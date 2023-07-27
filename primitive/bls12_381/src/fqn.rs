use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use crate::gt::Gt;
use crate::params::{
    BLS_X, FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1, FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ6_C2,
};
use zkstd::dress::extension_field::*;
use zkstd::dress::pairing::{bls12_range_field_pairing, peculiar_extension_field_operation};

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENSION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENSION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENSION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(
    Fq12,
    Fq2,
    Gt,
    G1Affine,
    PairingCoeff,
    BLS_X,
    BLS_X_IS_NEGATIVE
);

impl Fq2 {
    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    #[inline]
    pub fn lexicographically_largest(&self) -> bool {
        // If this element's c1 coefficient is lexicographically largest
        // then it is lexicographically largest. Otherwise, in the event
        // the c1 coefficient is zero and the c0 coefficient is
        // lexicographically largest, then this element is lexicographically
        // largest.

        self.0[1].lexicographically_largest()
            | self.0[1].is_zero() & self.0[0].lexicographically_largest()
    }

    pub fn sqrt(&self) -> Option<Self> {
        // Algorithm 9, https://eprint.iacr.org/2012/685.pdf
        // with constant time modifications.

        if self.is_zero() {
            return Some(Fq2::zero());
        }

        // a1 = self^((p - 3) / 4)
        let a1 = self.pow_vartime(&[
            0xee7fbfffffffeaaa,
            0x7aaffffac54ffff,
            0xd9cc34a83dac3d89,
            0xd91dd2e13ce144af,
            0x92c6e9ed90d2eb35,
            0x680447a8e5ff9a6,
        ]);
        // alpha = a1^2 * self = self^((p - 3) / 2 + 1) = self^((p - 1) / 2)
        let alpha = a1.square() * *self;
        // x0 = self^((p + 1) / 4)
        let x0 = a1 * *self;

        // In the event that alpha = -1, the element is order p - 1 and so
        // we're just trying to get the square of an element of the subfield
        // Fp. This is given by x0 * u, since u = sqrt(-1). Since the element
        // x0 = a + bu has b = 0, the solution is therefore au.
        let sqrt = match alpha == Fq2::one().neg() {
            true => Fq2([-x0.0[1], x0.0[0]]),
            false => {
                (alpha + Fq2::one()).pow_vartime(&[
                    0xdcff7fffffffd555,
                    0xf55ffff58a9ffff,
                    0xb39869507b587b12,
                    0xb23ba5c279c2895f,
                    0x258dd3db21a5d66b,
                    0xd0088f51cbff34d,
                ]) * x0
            }
        };
        match sqrt.square() == *self {
            true => Some(sqrt),
            false => None,
        }
    }

    /// Although this is labeled "vartime", it is only
    /// variable time with respect to the exponent. It
    /// is also not exposed in the public API.
    pub fn pow_vartime(&self, by: &[u64; 6]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res *= *self;
                }
            }
        }
        res
    }
}

// non common extension operation
peculiar_extension_field_operation!(
    Fq2,
    Fq6,
    Fq12,
    FROBENIUS_COEFF_FQ2_C1,
    FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ6_C2,
    FROBENIUS_COEFF_FQ12_C1,
    BLS_X_IS_NEGATIVE
);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;
    use zkstd::dress::field::field_test;

    field_test!(fq2_field, Fq2, 1000);
    field_test!(fq6_field, Fq6, 500);
    field_test!(fq12_field, Fq12, 100);

    #[test]
    fn fq2_mul_nonresidue_test() {
        let b = Fq2([Fq::one(); 2]);
        for _ in 0..1000 {
            let a = Fq2::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }

    #[test]
    fn fq6_mul_nonresidue_test() {
        let b = Fq6([Fq2::zero(), Fq2::one(), Fq2::zero()]);
        for _ in 0..1000 {
            let a = Fq6::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }

    #[test]
    fn fq6_mul_by_1_test() {
        for _ in 0..1000 {
            let c1 = Fq2::random(OsRng);
            let a = Fq6::random(OsRng);
            let b = Fq6([Fq2::zero(), c1, Fq2::zero()]);

            assert_eq!(a.mul_by_1(c1), a * b);
        }
    }

    #[test]
    fn fq6_mul_by_01_test() {
        for _ in 0..1000 {
            let c0 = Fq2::random(OsRng);
            let c1 = Fq2::random(OsRng);
            let a = Fq6::random(OsRng);
            let b = Fq6([c0, c1, Fq2::zero()]);

            assert_eq!(a.mul_by_01(c0, c1), a * b);
        }
    }

    #[test]
    fn fq12_mul_by_014_test() {
        for _ in 0..1000 {
            let c0 = Fq2::random(OsRng);
            let c1 = Fq2::random(OsRng);
            let c5 = Fq2::random(OsRng);
            let a = Fq12::random(OsRng);
            let b = Fq12([
                Fq6([c0, c1, Fq2::zero()]),
                Fq6([Fq2::zero(), c5, Fq2::zero()]),
            ]);

            assert_eq!(a.mul_by_014(c0, c1, c5), a * b);
        }
    }

    #[test]
    fn fq12_frobenius_map_test() {
        for _ in 0..1000 {
            let a = Fq12::random(OsRng);

            for i in 0..12 {
                let mut b = a;
                for _ in 0..i {
                    b = b.frobenius_map();
                }
                assert_eq!(a.frobenius_maps(i), b);
            }

            assert_eq!(a, a.frobenius_maps(12));
            assert_eq!(
                a,
                a.frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
            );
        }
    }
}
