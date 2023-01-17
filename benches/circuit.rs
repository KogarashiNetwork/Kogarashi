use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;
use zero_circuits::ConfidentialTransferCircuit;
use zero_crypto::common::Group;
use zero_jubjub::*;
use zero_plonk::prelude::*;

#[allow(unused_must_use)]
fn circuit(c: &mut Criterion) {
    let mut group = c.benchmark_group("circuit");

    let mut rng = StdRng::seed_from_u64(8349u64);
    let n = 1 << 14;
    let label = b"bench";
    group.bench_function("setup", |b| {
        b.iter(|| PublicParameters::setup(n, &mut rng).expect("failed to create pp"));
    });

    let pp = PublicParameters::setup(n, &mut rng).expect("failed to create pp");
    let (prover, verifier) = Compiler::compile::<ConfidentialTransferCircuit>(&pp, label)
        .expect("failed to compile circuit");
    let generator = GENERATOR_EXTENDED;
    let alice_private_key = JubJubScalar::random(&mut rng);
    let bob_private_key = JubJubScalar::random(&mut rng);
    let alice_public_key = generator * alice_private_key;
    let bob_public_key = generator * bob_private_key;
    let alice_balance = JubJubScalar::from(1500 as u64);
    let transfer_amount_b = JubJubScalar::from(800 as u64);
    let alice_after_balance = JubJubScalar::from(700 as u64);
    let alice_original_randomness = JubJubScalar::from(789 as u64);
    let randomness = JubJubScalar::from(123 as u64);
    let alice_left_encrypted_balance =
        (generator * alice_balance) + (alice_public_key * alice_original_randomness);
    let alice_right_encrypted_balance = generator * alice_original_randomness;
    let alice_left_encrypted_transfer_amount =
        (generator * transfer_amount_b) + (alice_public_key * randomness);
    let alice_right_encrypted_transfer_amount = generator * randomness;
    let bob_left_encrypted_transfer_amount =
        (generator * transfer_amount_b) + (bob_public_key * randomness);

    let (proof, public_inputs) = prover
        .prove(
            &mut rng,
            &ConfidentialTransferCircuit::new(
                JubJubAffine::from(alice_public_key),
                JubJubAffine::from(bob_public_key),
                JubJubAffine::from(alice_left_encrypted_balance),
                JubJubAffine::from(alice_right_encrypted_balance),
                JubJubAffine::from(alice_left_encrypted_transfer_amount),
                JubJubAffine::from(alice_right_encrypted_transfer_amount),
                JubJubAffine::from(bob_left_encrypted_transfer_amount),
                alice_private_key,
                transfer_amount_b,
                alice_after_balance,
                randomness,
            ),
        )
        .expect("failed to prove");

    group.bench_function("gen_proof", |b| {
        b.iter(|| {
            black_box(prover.prove(
                &mut rng,
                black_box(&ConfidentialTransferCircuit::new(
                    JubJubAffine::from(alice_public_key),
                    JubJubAffine::from(bob_public_key),
                    JubJubAffine::from(alice_left_encrypted_balance),
                    JubJubAffine::from(alice_right_encrypted_balance),
                    JubJubAffine::from(alice_left_encrypted_transfer_amount),
                    JubJubAffine::from(alice_right_encrypted_transfer_amount),
                    JubJubAffine::from(bob_left_encrypted_transfer_amount),
                    alice_private_key,
                    transfer_amount_b,
                    alice_after_balance,
                    randomness,
                )),
            ));
        })
    });
    group.bench_function("verify_proof", |b| {
        b.iter(|| {
            black_box(verifier.verify(black_box(&proof), black_box(&public_inputs)));
        })
    });
}

criterion_group!(
    name = bench_circuit;
    config = Criterion::default().sample_size(10);
    targets = circuit
);
criterion_main!(bench_circuit);
