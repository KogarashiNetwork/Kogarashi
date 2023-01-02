use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_bls12_381::{Fq12, Fr, G1Affine, G2Affine, G2PairingAffine, G2Projective};
use zero_crypto::behave::{Affine, Group, Pairing, PairingRange, PrimeField};
use zero_pairing::TatePairing;

#[test]
fn generator_pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;

    assert_eq!(Fq12::generator(), TatePairing::pairing(g1, g2));
}

#[test]
fn pairing_test() {
    let a = Fr::to_mont_form([1, 2, 3, 4]).invert().unwrap().square();
    let b = Fr::to_mont_form([5, 6, 7, 8]).invert().unwrap().square();
    let c = a * b;

    let g = G1Affine::from(G1Affine::ADDITIVE_GENERATOR * a);
    let h = G2Affine::from(G2Affine::ADDITIVE_GENERATOR * b);
    let p = TatePairing::pairing(g, h);

    let expected = G1Affine::from(G1Affine::ADDITIVE_GENERATOR * c);

    assert_eq!(
        p,
        TatePairing::pairing(expected, G2Affine::ADDITIVE_GENERATOR)
    );
    assert_eq!(
        p,
        TatePairing::pairing(
            G1Affine::ADDITIVE_GENERATOR,
            G2Affine::ADDITIVE_GENERATOR * c
        )
    );
}

#[test]
fn final_exp_test() {
    assert_eq!(Fq12::one().final_exp().unwrap(), Fq12::one());
}
