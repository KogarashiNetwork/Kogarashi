use zero_bls12_381::{Fq12, Fr, G1Affine, G2Affine, G2PairingAffine, G2Projective};
use zero_crypto::behave::{Affine, Group, Pairing, PairingRange, PrimeField};
use zero_pairing::TatePairing;

#[test]
fn generator_pairing_test() {
    let g1 = G1Affine::GENERATOR;
    let g2 = G2PairingAffine::from(G2Projective::GENERATOR);

    assert_eq!(Fq12::one(), TatePairing::pairing(g1, g2));
}

#[test]
fn pairing_test() {
    let g1 = G1Affine::GENERATOR;
    let g2 = G2Affine::GENERATOR;

    let a = Fr::one();
    let b = Fr::one();
    // let c = a * b;
    // let expected = TatePairing::pairing(g1 * c, G2PairingAffine::from(g2.to_projective()));

    let g = g1 * a;
    // let h = g2 * b;
    // let res = TatePairing::pairing(g, G2PairingAffine::from(h.to_projective()));

    // assert_eq!(res, expected)
}

#[test]
fn final_exp_test() {
    assert_eq!(Fq12::one().final_exp().unwrap(), Fq12::one());
}
