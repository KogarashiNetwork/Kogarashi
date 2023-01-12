use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_bls12_381::{Fq12, Fr, G1Affine, G2Affine, G2PairingAffine, Gt};
use zero_crypto::behave::{Group, Pairing, PairingRange};
use zero_pairing::TatePairing;

#[test]
fn generator_pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;
    let gt = Gt::ADDITIVE_GENERATOR;

    assert_eq!(gt, TatePairing::pairing(g1, g2));
}

#[test]
fn pairing_test() {
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;

    let mut rng = XorShiftRng::from_seed([
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ]);

    for _ in 0..10 {
        let a = Fr::random(&mut rng);
        let b = Fr::random(&mut rng);
        let c = a * b;

        let g = G1Affine::from(g1 * a);
        let h = G2Affine::from(g2 * b);
        let p = TatePairing::pairing(g, h);

        let expected = G1Affine::from(g1 * c);
        let test = G2Affine::from(g2 * c);

        assert_eq!(p, TatePairing::pairing(expected, g2));
        assert_eq!(p, TatePairing::pairing(g1, test));
    }
}

#[test]
fn final_exp_test() {
    assert_eq!(Fq12::one().final_exp(), Gt::ADDITIVE_IDENTITY);
}

#[test]
fn multi_miller_loop_test() {
    let mut rng = XorShiftRng::from_seed([
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ]);

    for _ in 0..5 {
        let a1 = G1Affine::ADDITIVE_GENERATOR;
        let b1 = G2Affine::ADDITIVE_GENERATOR;
        let a2 = G1Affine::from(a1 * Fr::random(&mut rng));
        let b2 = G2Affine::from(b1 * Fr::random(&mut rng));
        let a3 = G1Affine::from(a1 * Fr::random(&mut rng));
        let b3 = G2Affine::from(b1 * Fr::random(&mut rng));
        let a4 = G1Affine::from(a1 * Fr::random(&mut rng));
        let b4 = G2Affine::from(b1 * Fr::random(&mut rng));
        let a5 = G1Affine::from(a1 * Fr::random(&mut rng));
        let b5 = G2Affine::from(b1 * Fr::random(&mut rng));

        let b1_pairing = G2PairingAffine::from(b1);
        let b2_pairing = G2PairingAffine::from(b2);
        let b3_pairing = G2PairingAffine::from(b3);
        let b4_pairing = G2PairingAffine::from(b4);
        let b5_pairing = G2PairingAffine::from(b5);

        let expected = TatePairing::pairing(a1, b1)
            + TatePairing::pairing(a2, b2)
            + TatePairing::pairing(a3, b3)
            + TatePairing::pairing(a4, b4)
            + TatePairing::pairing(a5, b5);

        let test = TatePairing::multi_miller_loop(&[
            (a1, b1_pairing),
            (a2, b2_pairing),
            (a3, b3_pairing),
            (a4, b4_pairing),
            (a5, b5_pairing),
        ])
        .final_exp();

        assert_eq!(expected, test);
    }
}

#[test]
fn unitary_test() {
    let g = G1Affine::ADDITIVE_GENERATOR;
    let h = G2Affine::ADDITIVE_GENERATOR;

    let p = -TatePairing::pairing(g, h);
    let q = TatePairing::pairing(g, -h);
    let r = TatePairing::pairing(-g, h);

    assert_eq!(p, q);
    assert_eq!(q, r);
}
