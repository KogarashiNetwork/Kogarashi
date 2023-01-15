use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_plonk::FullcodecRng;
use pallet_plonk::Proof;
use zero_circuits::ConfidentialTransferTransaction;
use zero_elgamal::ConfidentialTransferPublicInputs;

/// Confidential transfer by coupling encrypted currency and plonk
pub trait ConfidentialTransfer<AccountId> {
    type EncryptedBalance: ConfidentialTransferPublicInputs;

    /// get account balance
    fn total_balance(who: &AccountId) -> Self::EncryptedBalance;

    /// trusted setup
    fn trusted_setup(who: &AccountId, val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// run confidential transfer transaction
    fn confidential_transfer(
        who: &AccountId,
        dest: &AccountId,
        proof: Proof,
        transaction_params: ConfidentialTransferTransaction<Self::EncryptedBalance>,
    ) -> DispatchResultWithPostInfo;
}
