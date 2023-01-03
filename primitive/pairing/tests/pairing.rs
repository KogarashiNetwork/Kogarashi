use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_bls12_381::{Fq12, Fr, G1Affine, G2Affine};
use zero_crypto::behave::{Group, Pairing, PairingRange};
use zero_pairing::TatePairing;

#[test]
fn generator_pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;
    let gt = Fq12::generator();

    assert_eq!(gt, TatePairing::pairing(g1, g2));
}

#[test]
fn pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;

    let mut rng = XorShiftRng::from_seed([
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ]);
    let mut rng_alt = XorShiftRng::from_seed([
        0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0,
    ]);

    for _ in 0..10 {
        let a = Fr::random(&mut rng);
        let b = Fr::random(&mut rng_alt);
        let c = a * b;

        let g = G1Affine::from(g1 * a);
        let h = G2Affine::from(g2 * b);
        let p = TatePairing::pairing(g, h);

        let expected = G1Affine::from(g1 * c);

        assert_eq!(p, TatePairing::pairing(expected, g2));
        assert_eq!(p, TatePairing::pairing(g1, g2 * c));
    }
}

#[test]
fn final_exp_test() {
    assert_eq!(Fq12::one().final_exp().unwrap(), Fq12::one());
}
