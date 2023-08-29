use zkstd::common::FftField;

use crate::{
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
};

pub(crate) struct Proof<F: FftField, H: FieldHasher<F, 2>, const N: usize, const BATCH_SIZE: usize>
{
    pub(crate) batch_tree: SparseMerkleTree<F, H, BATCH_SIZE>,
    pub(crate) t_merkle_proofs: Vec<MerkleProof<F, H, BATCH_SIZE>>,
    pub(crate) sender_receiver_in_state_merkle_proofs: Vec<MerkleProof<F, H, N>>,
    pub(crate) state_roots: Vec<(F, F)>,
}
