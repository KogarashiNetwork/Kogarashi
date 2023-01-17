use criterion::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::rngs::OsRng;
use rand::Rng;
use zero_bls12_381::{Fq, Fq12, Fq2, Fq6, Fr};
use zero_crypto::common::{FftField, Group, PrimeField};

fn bench_bls12_381_fr(c: &mut Criterion) {
    let mut group = c.benchmark_group("bls12_381_fq");

    let x = Fr::random(OsRng);
    let y = Fr::random(OsRng);
    let p = rand::thread_rng().gen::<u64>();

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
    group.bench_function("pow", |b| {
        b.iter(|| black_box(black_box(x).pow(p)));
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

fn bls12_381_field_benchmark(c: &mut Criterion) {
    run::<Fq>(c);
    run::<Fq2>(c);
    run::<Fq6>(c);
    run::<Fq12>(c);
}

criterion_group!(
    bls12_381_field,
    bench_bls12_381_fr,
    bls12_381_field_benchmark
);
criterion_main!(bls12_381_field);
