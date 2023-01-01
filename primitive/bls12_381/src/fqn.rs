use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use crate::params::{
    BLS_X, BLS_X_IS_NEGATIVE, FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1,
    FROBENIUS_COEFF_FQ6_C1,
};
use zero_crypto::dress::extension_field::*;
use zero_crypto::dress::pairing::{bls12_range_field_pairing, peculiar_extension_field_operation};

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENTION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(Fq12, Fq2, G1Affine, PairingCoeff, BLS_X, BLS_X_IS_NEGATIVE);

// non common extension operation
peculiar_extension_field_operation!(
    Fq2,
    Fq6,
    Fq12,
    FROBENIUS_COEFF_FQ2_C1,
    FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ12_C1,
    BLS_X_IS_NEGATIVE
);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;
    use zero_crypto::dress::field::field_test;

    field_test!(fq2_field, Fq2, 1000);
    field_test!(fq6_field, Fq6, 500);
    field_test!(fq12_field, Fq12, 500);

    #[test]
    fn fq2_mul_nonresidue_test() {
        let b = Fq2([Fq::one(); TWO_DEGREE_EXTENTION_LIMBS_LENGTH]);
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

            assert_eq!(a, a.frobenius_map(12));
        }
    }

    #[test]
    fn fq12_pow_test() {
        for _ in 0..1000 {
            let a = Fq12::random(OsRng);

            assert_eq!(a, a.frobenius_map(12));
        }
    }
}
