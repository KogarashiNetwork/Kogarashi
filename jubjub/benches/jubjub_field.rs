use criterion::{black_box, criterion_group, criterion_main, Criterion};

use jub_jub::Fp;
use zkstd::common::{GroupParams, OsRng, PrimeField};

fn bench_fp(c: &mut Criterion) {
    let mut group = c.benchmark_group("jubjub_fp");

    let x = Fp::random(OsRng);
    let y = Fp::random(OsRng);

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

criterion_group!(jubjub_field, bench_fp);
criterion_main!(jubjub_field);
