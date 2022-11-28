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
    use rand_xorshift::XorShiftRng;
    use zero_bls12_381::{Fr, G1Projective, G2Projective};
    use zero_crypto::common::*;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_poly(k: u32)(bytes in vec![[any::<u8>(); 16]; 1 << k as usize]) -> Polynomial<Fr> {
            Polynomial((0..(1 << k)).map(|i| Fr::random(XorShiftRng::from_seed(bytes[i]))).collect::<Vec<Fr>>())
        }
    }

    fn evaluate_g1(poly: &Polynomial<Fr>, base: G1Projective) -> G1Projective {
        let mut acc = G1Projective::IDENTITY;
        let mut exp = G1Projective::IDENTITY;

        for coeff in poly.0.iter().rev() {
            acc += base * *coeff;
            exp += base
        }
        acc
    }

    fn evaluate_g2(poly: &Polynomial<Fr>, base: G2Projective) -> G2Projective {
        let mut acc = G2Projective::IDENTITY;
        let mut exp = G2Projective::IDENTITY;

        for coeff in poly.0.iter().rev() {
            acc += base * *coeff;
            exp += base
        }
        acc
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(3))]
        #[test]
        fn kzg_commit_test(r in arb_fr(), poly in arb_poly(5)) {
            let k = 5;
            let g1_g = G1Projective::GENERATOR * r;
            let g2_g = G2Projective::GENERATOR * r;
            let keypair = KeyPair::<KzgCommitment>::setup(k, r);
            let g1_commitment = keypair.commit_to_g1(&poly);
            let g2_commitment = keypair.commit_to_g2(&poly);
            let g1_evaluation = evaluate_g1(&poly, g1_g);
            let g2_evaluation = evaluate_g2(&poly, g2_g);

            assert_eq!(g1_commitment, g1_evaluation);
            assert_eq!(g2_commitment, g2_evaluation);
        }
    }
}
