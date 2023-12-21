use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bn_254::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use rand_core::OsRng;
use zkstd::common::{BNAffine, BNProjective, Group};

fn bench_g1_affine(c: &mut Criterion) {
    let mut group = c.benchmark_group("g1_affine");
    let mut rng = OsRng;

    let p1 = G1Affine::random(&mut rng);
    let p2 = G1Affine::random(&mut rng);
    let k = Fr::random(&mut rng);

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
    let mut rng = OsRng;

    let p1 = G1Projective::random(&mut rng);
    let p2 = G1Projective::random(&mut rng);
    let k = Fr::random(&mut rng);

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

fn bench_g2_affine(c: &mut Criterion) {
    let mut group = c.benchmark_group("g2_affine");
    let mut rng = OsRng;

    let p1 = G2Affine::random(&mut rng);
    let p2 = G2Affine::random(&mut rng);
    let k = Fr::random(&mut rng);

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

fn bench_g2_projective(c: &mut Criterion) {
    let mut group = c.benchmark_group("g2_projective");
    let mut rng = OsRng;

    let p1 = G2Projective::random(&mut rng);
    let p2 = G2Projective::random(&mut rng);
    let k = Fr::random(&mut rng);

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

criterion_group!(
    bn_254_curve,
    bench_g1_affine,
    bench_g1_projective,
    bench_g2_affine,
    bench_g2_projective
);
criterion_main!(bn_254_curve);
