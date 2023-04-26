use criterion::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::rngs::OsRng;
use zero_crypto::common::{Curve, CurveGroup, Group};
use zero_jubjub::Fp;
use zero_jubjub::JubjubExtended;

fn bench_jubjub_extended(c: &mut Criterion) {
    let mut group = c.benchmark_group("jubjub_extended");

    let p1 = JubjubExtended::random(OsRng);
    let p2 = JubjubExtended::random(OsRng);
    let k = Fp::random(OsRng);

    group.bench_function("add", |b| {
        b.iter(|| black_box(black_box(p1) + black_box(p2)));
    });
    group.bench_function("sub", |b| {
        b.iter(|| black_box(black_box(p1) - black_box(p2)));
    });
    group.bench_function("double", |b| {
        b.iter(|| black_box(black_box(p1).double()));
    });
    group.bench_function("scalar", |b| {
        b.iter(|| black_box(black_box(p1) * black_box(k)));
    });
}

criterion_group!(jubjub_curve, bench_jubjub_extended);
criterion_main!(jubjub_curve);
