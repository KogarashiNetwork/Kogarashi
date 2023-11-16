#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod ivc;
mod pedersen;
mod proof;
mod prover;
mod relaxed_r1cs;
#[allow(dead_code)]
mod transcript;
mod verifier;

pub use ivc::Ivc;
pub use pedersen::PedersenCommitment;
pub use proof::RecursiveProof;
pub use prover::Prover;
pub use relaxed_r1cs::RelaxedR1cs;
pub use verifier::Verifier;