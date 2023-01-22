use zero_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use zero_crypto::common::Commitment;

// kate polynomial commiment
pub struct KzgCommitment;

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

    use rand_core::OsRng;
    use zero_bls12_381::{Fr, G1Projective};
    use zero_crypto::common::*;

    fn arb_fr() -> Fr {
        Fr::random(OsRng)
    }

    fn arb_poly(k: u32) -> Polynomial<Fr> {
        Polynomial(
            (0..(1 << k))
                .map(|_| Fr::random(OsRng))
                .collect::<Vec<Fr>>(),
        )
    }

    fn evaluate(poly: &Polynomial<Fr>, at: Fr) -> G1Projective {
        let mut acc = G1Projective::ADDITIVE_IDENTITY;
        let mut identity = Fr::ADDITIVE_IDENTITY;

        for coeff in poly.0.iter().rev() {
            let interm = G1Projective::ADDITIVE_GENERATOR * identity;
            let product = interm * *coeff;
            acc += product;
            identity *= at;
        }
        acc
    }

    #[test]
    fn kzg_commit_test() {
        let r = arb_fr();
        let poly = arb_poly(5);
        let k = 5;
        let keypair = KeyPair::<KzgCommitment>::setup(k, r);
        let g1_commitment = keypair.commit(&poly);
        let g1_evaluation = evaluate(&poly, r);

        assert_eq!(g1_commitment, g1_evaluation);
    }
}
