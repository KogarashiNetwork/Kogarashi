use crate::types::*;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use sp_std::vec::Vec;
use zero_bls12_381::Fr;

/// Abstraction over a plonk zk-SNARKs system
pub trait Plonk<AccountId> {
    /// The plonk circuit customized by developer
    type CustomCircuit: Circuit;

    /// The public parameters generation function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn trusted_setup(who: &AccountId, val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn verify(who: &AccountId, proof: Proof, public_inputs: Vec<Fr>) -> DispatchResultWithPostInfo;
}
