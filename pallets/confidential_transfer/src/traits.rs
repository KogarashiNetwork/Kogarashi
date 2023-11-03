// < HB SBP M2 review:
//
// generally these kind of definitions are contained in a types.rs, leaving the implementations to impls.rs
//
// >
use crate::circuit::ConfidentialTransferTransaction;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_plonk::FullcodecRng;
use pallet_plonk::Proof;
use she_elgamal::ConfidentialTransferPublicInputs;
use zkstd::common::Pairing;
use zkstd::common::TwistedEdwardsAffine;

/// Confidential transfer by coupling encrypted currency and plonk
pub trait ConfidentialTransfer<AccountId, P: Pairing, A: TwistedEdwardsAffine> {
    type EncryptedBalance: ConfidentialTransferPublicInputs<A>;

    /// get account balance
    fn total_balance(who: &AccountId) -> Self::EncryptedBalance;

    // trusted setup
    // < HB SBP M2 review:
    //
    // I would avoid using rust primitives as parameter types. I would suggest to define a proper type for val.
    //
    // >
    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// run confidential transfer transaction
    fn confidential_transfer(
        who: &AccountId,
        dest: &AccountId,
        proof: Proof<P>,
        transaction_params: ConfidentialTransferTransaction<Self::EncryptedBalance, A>,
    ) -> DispatchResultWithPostInfo;
}
