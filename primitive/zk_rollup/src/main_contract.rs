use sp_std::marker::PhantomData;

use red_jubjub::PublicKey;
use zkstd::common::{vec, FftField, Vec};

use crate::{
    domain::Transaction, merkle_tree::MerkleProof, operator::Batch, poseidon::FieldHasher,
    proof::Proof, verifier_contract::VerifierContract,
};

#[derive(Default)]
pub(crate) struct MainContract<
    F: FftField,
    H: FieldHasher<F, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    address: PublicKey,
    pub(crate) rollup_state_root: F,
    deposits: Vec<Transaction>,
    verifier_contract: VerifierContract<F, H, N, BATCH_SIZE>,
    pub(crate) calldata: Vec<Batch<F, H, N, BATCH_SIZE>>,

    marker: PhantomData<H>,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize, const BATCH_SIZE: usize>
    MainContract<F, H, N, BATCH_SIZE>
{
    pub fn deposit(&mut self, t: Transaction) {
        self.deposits.push(t);
    }

    pub fn withdraw(
        &self,
        // l2_burn_merkle_proof: MerkleProof<F, H, N>,
        batch_root: F,
        transaction: Transaction,
        l1_address: PublicKey,
    ) {
        // merkle_verify(l2_burn_merkle_proof, batch_root);
        // send(transaction.amount, l1_address);
    }

    pub fn update_state(&mut self, new_state_root: F) {
        self.rollup_state_root = new_state_root;
    }
    pub fn add_batch(
        &mut self,
        proof: Proof<F, H, N, BATCH_SIZE>,
        compressed_batch_data: Batch<F, H, N, BATCH_SIZE>,
    ) {
        assert!(self.verifier_contract.verify_proof(proof));
        self.update_state(compressed_batch_data.final_root());
        self.calldata.push(compressed_batch_data);
    }

    pub fn check_balance(&self, merkle_proof: MerkleProof<F, H, N>) -> u64 {
        // merkle_verify(merkle_proof, self.rollup_state_root);
        // get_balance()
        0
    }

    pub fn deposits(&self) -> &Vec<Transaction> {
        &self.deposits
    }

    pub(crate) fn new(rollup_state_root: F, address: PublicKey) -> Self {
        Self {
            address,
            rollup_state_root,
            deposits: vec![],
            marker: PhantomData,
            verifier_contract: VerifierContract::default(),
            calldata: vec![],
        }
    }

    pub(crate) fn address(&self) -> PublicKey {
        self.address
    }
}
