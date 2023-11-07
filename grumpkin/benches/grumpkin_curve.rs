use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bn_254::Fr;
use grumpkin::{G1Affine, G1Projective};
use rand_core::OsRng;
use zkstd::common::{CurveGroup, Group};

fn bench_g1_affine(c: &mut Criterion) {
    let mut group = c.benchmark_group("g1_affine");

    let p1 = G1Affine::random(OsRng);
    let p2 = G1Affine::random(OsRng);
    let k = Fr::random(OsRng);

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

fn bench_g1_projective(c: &mut Criterion) {
    let mut group = c.benchmark_group("g1_projective");

    let p1 = G1Projective::random(OsRng);
    let p2 = G1Projective::random(OsRng);
    let k = Fr::random(OsRng);

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

criterion_group!(grumpkin_curve, bench_g1_affine, bench_g1_projective,);
criterion_main!(grumpkin_curve);
