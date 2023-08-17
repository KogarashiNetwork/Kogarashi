use std::{hash::Hash, marker::PhantomData};

pub(crate) type TreeHash = [u8; 32];
pub(crate) struct MerkleProof(Vec<TreeHash>);
pub(crate) struct MerkleTree<T: Hash> {
    size: u64,
    root: TreeHash,
    leaves: Vec<TreeHash>,
    _hasher: PhantomData<T>,
}
