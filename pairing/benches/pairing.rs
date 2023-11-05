use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bls_12_381::{Fq12, Fr, G1Affine, G2Affine, G2PairingAffine};
use ec_pairing::TatePairing;
use zkstd::common::{GroupParams, OsRng, Pairing, PairingRange};

fn pairing(c: &mut Criterion) {
    let mut group = c.benchmark_group("pairing");
    let g1 = G1Affine::ADDITIVE_GENERATOR;
    let g2 = G2Affine::ADDITIVE_GENERATOR;

    group.bench_function("tate", |b| {
        b.iter(|| TatePairing::pairing(g1, g2));
    });

    group.bench_function("final_exp", |b| {
        b.iter(|| Fq12::one().final_exp());
    });

    group.bench_function("miller_loop", |b| {
        b.iter(|| TatePairing::miller_loop(g1, g2));
    });

    let a1 = G1Affine::ADDITIVE_GENERATOR;
    let b1 = G2Affine::ADDITIVE_GENERATOR;
    let a2 = G1Affine::from(a1 * Fr::random(OsRng));
    let b2 = G2Affine::from(b1 * Fr::random(OsRng));
    let a3 = G1Affine::from(a1 * Fr::random(OsRng));
    let b3 = G2Affine::from(b1 * Fr::random(OsRng));
    let a4 = G1Affine::from(a1 * Fr::random(OsRng));
    let b4 = G2Affine::from(b1 * Fr::random(OsRng));
    let a5 = G1Affine::from(a1 * Fr::random(OsRng));
    let b5 = G2Affine::from(b1 * Fr::random(OsRng));

    let b1_pairing = G2PairingAffine::from(b1);
    let b2_pairing = G2PairingAffine::from(b2);
    let b3_pairing = G2PairingAffine::from(b3);
    let b4_pairing = G2PairingAffine::from(b4);
    let b5_pairing = G2PairingAffine::from(b5);

    group.bench_function("multi_miller_loop", |b| {
        b.iter(|| {
            black_box(TatePairing::multi_miller_loop(black_box(&[
                (a1, b1_pairing.clone()),
                (a2, b2_pairing.clone()),
                (a3, b3_pairing.clone()),
                (a4, b4_pairing.clone()),
                (a5, b5_pairing.clone()),
            ])))
        });
    });
}

criterion_group!(bench_pairing, pairing);
criterion_main!(bench_pairing);
