#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod batch_circuit;
mod db;
mod domain;
mod merkle_tree;
mod operator;
mod poseidon;
mod redjubjub_circuit;

pub use batch_circuit::BatchCircuit;
pub use domain::{Transaction, TransactionData};
pub use operator::{Batch, BatchGetter, RollupOperator};
pub use poseidon::{FieldHasher, Poseidon};
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng as FullcodecRng;

pub(crate) fn get_rng() -> FullcodecRng {
    FullcodecRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ])
}
