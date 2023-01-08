use crate::mock::DummyCircuit;
use crate::types::{JubJubScalar, PublicParameters};
use rand::rngs::StdRng;
use rand::SeedableRng;
use zero_crypto::behave::Group;
use zero_plonk::prelude::Compiler;

#[test]
fn default_test() {
    let rng = &mut StdRng::seed_from_u64(8349u64);

    let n = 1 << 9;
    let label = b"demo";
    let pp = PublicParameters::setup(n, rng).expect("failed to create pp");

    let (prover, verifier) =
        Compiler::compile::<DummyCircuit>(&pp, label).expect("failed to compile circuit");

    let a = JubJubScalar::random(rng.clone());
    let (proof, public_inputs) = prover
        .prove(rng, &DummyCircuit::new(a))
        .expect("failed to prove");

    verifier
        .verify(&proof, &public_inputs)
        .expect("failed to verify proof");
}
