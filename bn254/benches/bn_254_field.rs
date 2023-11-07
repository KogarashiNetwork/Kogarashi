use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bn_254::{Fq, Fq12, Fq2, Fq6, Fr};
use rand_core::OsRng;
use zkstd::common::{Group, PrimeField};

fn bench_bn_254_fr(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_bn_254_fr");

    let x = Fr::random(OsRng);
    let y = Fr::random(OsRng);

    group.bench_function("add", |b| {
        b.iter(|| black_box(black_box(x) + black_box(y)));
    });
    group.bench_function("sub", |b| {
        b.iter(|| black_box(black_box(x) - black_box(y)));
    });
    group.bench_function("double", |b| {
        b.iter(|| black_box(black_box(x).double()));
    });
    group.bench_function("mul", |b| {
        b.iter(|| black_box(black_box(x) * black_box(y)));
    });
    group.bench_function("square", |b| {
        b.iter(|| black_box(black_box(x).square()));
    });
    group.bench_function("invert", |b| {
        b.iter(|| black_box(black_box(x).invert()));
    });
}

fn run<PF: PrimeField>(c: &mut Criterion) {
    let mut group = c.benchmark_group(std::any::type_name::<PF>());

    let x = PF::random(OsRng);
    let y = PF::random(OsRng);

    group.bench_function("add", |b| {
        b.iter(|| black_box(black_box(x) + black_box(y)));
    });
    group.bench_function("sub", |b| {
        b.iter(|| black_box(black_box(x) - black_box(y)));
    });
    group.bench_function("double", |b| {
        b.iter(|| black_box(black_box(x).double()));
    });
    group.bench_function("mul", |b| {
        b.iter(|| black_box(black_box(x) * black_box(y)));
    });
    group.bench_function("square", |b| {
        b.iter(|| black_box(black_box(x).square()));
    });
    group.bench_function("invert", |b| {
        b.iter(|| black_box(black_box(x).invert()));
    });
}

fn bn_254_field_benchmark(c: &mut Criterion) {
    run::<Fq>(c);
    run::<Fq2>(c);
    run::<Fq6>(c);
    run::<Fq12>(c);
}

criterion_group!(bn_254_field, bench_bn_254_fr, bn_254_field_benchmark);
criterion_main!(bn_254_field);
