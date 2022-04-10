//! A smart contract which demonstrates behavior of the `self.env().transfer()` function.
//! It transfers some of it's balance to the caller.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::new_without_default)]

use ink_lang as ink;

#[ink::contract]
pub mod give_me {
    /// No storage is needed for this simple contract.
    #[ink(storage)]
    pub struct GiveMe {}

    impl GiveMe {
        /// Creates a new instance of this contract.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {}
        }

        /// Transfers `value` amount of tokens to the caller.
        ///
        /// # Errors
        ///
        /// - Panics in case the requested transfer exceeds the contract balance.
        /// - Panics in case the requested transfer would have brought this
        ///   contract's balance below the minimum balance (i.e. the chain's
        ///   existential deposit).
        /// - Panics in case the transfer failed for another reason.
        #[ink(message)]
        pub fn give_me(&mut self, value: Balance) {
            ink_env::debug_println!("requested value: {}", value);
            ink_env::debug_println!("contract balance: {}", self.env().balance());

            assert!(value <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "requested transfer failed. this can be the case if the contract does not\
                     have sufficient free funds or if the transfer would have brought the\
                     contract's balance below minimum balance."
                )
            }
        }

        /// Asserts that the token amount sent as payment with this call
        /// is exactly `10`. This method will fail otherwise, and the
        /// transaction would then be reverted.
        ///
        /// # Note
        ///
        /// The method needs to be annotated with `payable`; only then it is
        /// allowed to receive value as part of the call.
        #[ink(message, payable, selector = 0xCAFEBABE)]
        pub fn was_it_ten(&self) {
            ink_env::debug_println!(
                "received payment: {}",
                self.env().transferred_value()
            );
            assert!(self.env().transferred_value() == 10, "payment was not ten");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn transfer_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            let mut give_me = create_contract(contract_balance);

            // when
            set_sender(accounts.eve);
            set_balance(accounts.eve, 0);
            give_me.give_me(80);

            // then
            assert_eq!(get_balance(accounts.eve), 80);
        }

        #[ink::test]
        #[should_panic(expected = "insufficient funds!")]
        fn transfer_fails_insufficient_funds() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            let mut give_me = create_contract(contract_balance);

            // when
            set_sender(accounts.eve);
            give_me.give_me(120);

            // then
            // `give_me` must already have panicked here
        }

        #[ink::test]
        fn test_transferred_value() {
            // given
            let accounts = default_accounts();
            let give_me = create_contract(100);

            // when
            // Push the new execution context which sets Eve as caller and
            // the `mock_transferred_value` as the value which the contract
            // will see as transferred to it.
            set_sender(accounts.eve);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(10);

            // then
            // there must be no panic
            give_me.was_it_ten();
        }

        #[ink::test]
        #[should_panic(expected = "payment was not ten")]
        fn test_transferred_value_must_fail() {
            // given
            let accounts = default_accounts();
            let give_me = create_contract(100);

            // when
            // Push the new execution context which sets Eve as caller and
            // the `mock_transferred_value` as the value which the contract
            // will see as transferred to it.
            set_sender(accounts.eve);
            ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(13);

            // then
            give_me.was_it_ten();
        }

        /// Creates a new instance of `GiveMe` with `initial_balance`.
        ///
        /// Returns the `contract_instance`.
        fn create_contract(initial_balance: Balance) -> GiveMe {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            GiveMe::new()
        }

        fn contract_id() -> AccountId {
            ink_env::test::callee::<ink_env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }

        fn default_accounts(
        ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }
    }
}
