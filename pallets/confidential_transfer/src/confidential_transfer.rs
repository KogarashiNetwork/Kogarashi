use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_encrypted_balance::EncryptedCurrency;
use pallet_plonk::Plonk;
use pallet_plonk::Proof;
use zero_circuits::ConfidentialTransferTransaction;

/// Confidential transfer by coupling encrypted currency and plonk
pub trait ConfidentialTransfer<AccountId>: EncryptedCurrency<AccountId> + Plonk<AccountId> {
    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn confidential_transfer(
        who: &AccountId,
        dest: &AccountId,
        proof: Proof,
        transaction_params: ConfidentialTransferTransaction<Self::EncryptedBalance>,
    ) -> DispatchResultWithPostInfo;
}
