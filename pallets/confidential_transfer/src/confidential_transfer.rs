use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_plonk::Proof;
use zero_circuits::ConfidentialTransferTransaction;
use zero_elgamal::ConfidentialTransferPublicInputs;

/// Confidential transfer by coupling encrypted currency and plonk
pub trait ConfidentialTransfer<AccountId> {
    type EncryptedBalance: ConfidentialTransferPublicInputs;

    /// run confidential transfer transaction
    fn confidential_transfer(
        who: &AccountId,
        dest: &AccountId,
        proof: Proof,
        transaction_params: ConfidentialTransferTransaction<Self::EncryptedBalance>,
    ) -> DispatchResultWithPostInfo;
}
