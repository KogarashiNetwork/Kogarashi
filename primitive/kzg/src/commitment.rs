use zero_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use zero_crypto::common::Commitment;

// kate polynomial commiment
pub struct KzgCommitment {}

impl Commitment for KzgCommitment {
    // affine point group
    type G1Affine = G1Affine;

    // projective point group
    type G1Projective = G1Projective;

    // the other affine point group
    type G2Affine = G2Affine;

    // the other affine point group
    type G2Projective = G2Projective;

    // scalar point of point group
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
    use zero_bls12_381::Fr;
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

    fn evaluate<P: Projective>(poly: &Polynomial<P::ScalarField>, at: P::ScalarField) -> P {
        let mut acc = P::IDENTITY;
        let mut identity = P::ScalarField::IDENTITY;

        for coeff in poly.0.iter().rev() {
            let interm = P::GENERATOR * identity;
            let product = interm * *coeff;
            acc += product;
            identity *= at;
        }
        acc
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(3))]
        #[test]
        fn kzg_commit_test(r in arb_fr(), poly in arb_poly(5)) {
            let k = 5;
            let keypair = KeyPair::<KzgCommitment>::setup(k, r);
            let g1_commitment = keypair.commit(&poly);
            let g1_evaluation = evaluate(&poly, r);

            assert_eq!(g1_commitment, g1_evaluation);
        }
    }
}
