use sp_runtime::DispatchResult;

/// Abstraction over a fungible assets system.
pub trait EncryptedCurrency<AccountId> {
    /// The balance of an account.
    type EncryptedBalance;
    /// Transfer some liquid free balance to another staker.
    ///
    /// This is a very high-level function. It will ensure all appropriate fees are paid
    /// and no imbalance in the system remains.
    fn transfer(
        source: &AccountId,
        dest: &AccountId,
        value: Self::EncryptedBalance,
    ) -> DispatchResult;
}
