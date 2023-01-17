use sp_runtime::DispatchResult;

/// Abstraction over a fungible assets system.
pub trait EncryptedCurrency<AccountId, EncryptedBalance> {
    fn total_balance(who: &AccountId) -> EncryptedBalance;

    /// Transfer some liquid free balance to another staker.
    ///
    /// This is a very high-level function. It will ensure all appropriate fees are paid
    /// and no imbalance in the system remains.
    fn transfer(
        source: &AccountId,
        dest: &AccountId,
        sender_amount: EncryptedBalance,
        recipient_amount: EncryptedBalance,
    ) -> DispatchResult;

    /// Deposit some `value` into the free balance of `who`, possibly creating a new account.
    ///
    /// This function is a no-op if:
    /// - the `value` to be deposited is zero; or
    /// - the `value` to be deposited is less than the required ED and the account does not yet exist; or
    /// - the deposit would necessitate the account to exist and there are no provider references; or
    /// - `value` is so large it would cause the balance of `who` to overflow.
    fn deposit_creating(who: &AccountId, value: EncryptedBalance) -> DispatchResult;
}
