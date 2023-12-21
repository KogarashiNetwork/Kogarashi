#![doc = include_str!("../README.md")]
#![allow(unused_variables, dead_code)]

mod circuit;
mod function;
mod gadget;
mod hash;
mod ivc;
mod pedersen;
mod proof;
mod prover;
mod relaxed_r1cs;
mod verifier;

mod driver;
#[cfg(test)]
mod test;

pub use driver::{Bn254Driver, GrumpkinDriver};
pub use function::FunctionCircuit;
pub use ivc::{Ivc, PublicParams};
pub use pedersen::PedersenCommitment;
pub use proof::RecursiveProof;
pub use prover::Prover;
pub use relaxed_r1cs::R1csShape;
pub use verifier::Verifier;
