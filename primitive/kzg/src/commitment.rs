use zero_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use zero_crypto::common::Commitment;

pub struct KzgCommitment {}

impl Commitment for KzgCommitment {
    type G1Affine = G1Affine;

    type G1Projective = G1Projective;

    type G2Affine = G2Affine;

    type G2Projective = G2Projective;

    type ScalarField = Fr;
}

#[cfg(test)]
mod tests {
    use super::KzgCommitment;
    use crate::keypair::KeyPair;
    use crate::poly::Polynomial;

    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_core::RngCore;
    use rand_xorshift::XorShiftRng;
    use zero_bls12_381::{Fr, G1Projective, G2Projective};
    use zero_crypto::common::*;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    fn evaluate_g1(poly: Polynomial<Fr>, base: G1Projective) -> G1Projective {
        let mut acc = G1Projective::IDENTITY;
        let mut exp = G1Projective::IDENTITY;

        for coeff in poly.0.iter().rev() {
            acc += base * *coeff;
            exp += base
        }
        acc
    }

    fn evaluate_g2(poly: Polynomial<Fr>, base: G2Projective) -> G2Projective {
        let mut acc = G2Projective::IDENTITY;
        let mut exp = G2Projective::IDENTITY;

        for coeff in poly.0.iter().rev() {
            acc += base * *coeff;
            exp += base
        }
        acc
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn commit_g1_test(r in arb_fr()) {
            let k = 10;
            let keypair = KeyPair::<KzgCommitment>::setup(10, r);
        }
    }
}
