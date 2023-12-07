#![doc = include_str!("../README.md")]
#![allow(unused_variables, dead_code)]

mod circuit;
mod driver;
mod function;
mod gadget;
mod hash;
mod ivc;
mod pedersen;
mod proof;
mod prover;
mod relaxed_r1cs;
mod verifier;

#[cfg(test)]
mod test;

pub use ivc::Ivc;
pub use pedersen::PedersenCommitment;
pub use proof::RecursiveProof;
pub use prover::Prover;
pub use relaxed_r1cs::RelaxedR1cs;
pub use verifier::Verifier;
