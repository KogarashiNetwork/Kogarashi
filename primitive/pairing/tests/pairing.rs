use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_bls12_381::{Fq12, Fr, G1Affine, G2Affine, G2PairingAffine, G2Projective};
use zero_crypto::behave::{Affine, Group, Pairing, PairingRange};
use zero_pairing::TatePairing;

#[test]
fn generator_pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2PairingAffine::from(G2Projective::ADDITIVE_GENERATOR);

    assert_eq!(Fq12::one(), TatePairing::pairing(g1, g2));
}

#[test]
fn pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;

    let a = Fr::random(XorShiftRng::from_seed([
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ]));
    let b = Fr::random(XorShiftRng::from_seed([
        0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0,
    ]));
    let c = a * b;
    let expected = TatePairing::pairing(g1 * c, G2PairingAffine::from(g2.to_projective()));

    let g = g1 * a;
    let h = g2 * b;
    let res = TatePairing::pairing(g, G2PairingAffine::from(h.to_projective()));

    println!("{:?}", res);
    assert_eq!(res, expected)
}

#[test]
fn final_exp_test() {
    assert_eq!(Fq12::one().final_exp().unwrap(), Fq12::one());
}
