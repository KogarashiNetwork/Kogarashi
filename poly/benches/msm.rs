use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use bls_12_381::{Fr, G1Affine};
use poly_commit::msm_curve_addition;
use zkstd::common::{Group, OsRng};

fn msm(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm");

    for i in 8..=14 {
        let p = vec![G1Affine::from(G1Affine::random(OsRng)); 1 << i];
        let k = vec![Fr::random(OsRng); 1 << i];

        // 2^{8-14} points
        group.bench_function(BenchmarkId::new("msm_based", i), |b| {
            b.iter(|| black_box(msm_curve_addition(black_box(&p), black_box(&k))));
        });
    }
}

criterion_group!(bench_msm, msm);
criterion_main!(bench_msm);
