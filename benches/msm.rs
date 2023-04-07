use criterion::{black_box, BenchmarkId};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::rngs::OsRng;
use zero_bls12_381::Fr;
use zero_bls12_381::G1Affine;
use zero_crypto::common::{CurveGroup, Group};
use zero_pairing::{msm_variable_base, TatePairing};

fn msm(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm");

    for i in 8..=14 {
        let p = vec![G1Affine::from(G1Affine::random(OsRng)); 1 << i];
        let k = vec![Fr::random(OsRng); 1 << i];

        // 8-18 points
        group.bench_function(BenchmarkId::new("msm_based", i), |b| {
            b.iter(|| {
                black_box(msm_variable_base::<TatePairing>(
                    black_box(&p),
                    black_box(&k),
                ))
            });
        });
    }
}

criterion_group!(bench_msm, msm);
criterion_main!(bench_msm);
