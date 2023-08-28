#![allow(dead_code)]
#![allow(unused_variables)]
mod db;
mod main_contract;
mod merkle_tree;
mod operator;
mod poseidon;
mod proof;
mod redjubjub_circuit;
mod verifier_contract;

#[cfg(test)]
mod tests {

    use jub_jub::{Fp, JubjubExtended};
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{PublicKey, SecretKey};
    use zkstd::common::{CurveGroup, Group, SigUtils};

    use crate::{
        main_contract::MainContract,
        operator::{RollupOperator, Transaction, TransactionData, UserData},
        poseidon::Poseidon,
    };

    #[test]
    fn sig_utils() {
        let mut rng = StdRng::seed_from_u64(8349u64);
        let secret = SecretKey::new(Fp::random(&mut rng));
        let td = TransactionData::new(
            secret.to_public_key(),
            PublicKey::new(JubjubExtended::random(&mut rng)),
            10,
        );

        let user = UserData::new(0, 10, secret.to_public_key());

        let t = td.signed(secret, &mut rng);

        let td_bytes = td.to_bytes();
        let td_back = TransactionData::from_bytes(td_bytes).unwrap();
        assert_eq!(td, td_back);

        let t_bytes = t.to_bytes();
        let t_back = Transaction::from_bytes(t_bytes).unwrap();
        assert_eq!(t, t_back);

        let user_bytes = user.to_bytes();
        let user_back = UserData::from_bytes(user_bytes).unwrap();
        assert_eq!(user, user_back);
    }

    #[test]
    fn test_deposit_and_transfer_works() {
        let mut rng = StdRng::seed_from_u64(8349u64);
        let poseidon = Poseidon::<Fp, 2>::new();
        let mut operator = RollupOperator::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT>::new(&poseidon);
        let mut contract = MainContract::<Fp>::new(
            operator.state_root(),
            PublicKey::new(JubjubExtended::random(&mut rng)),
        );

        const ACCOUNT_LIMIT: usize = 2;

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();

        let root_before_dep = operator.state_root();
        assert_eq!(
            root_before_dep,
            Fp::from_hex("0x0d3aa68d8765c3e6dc78d34ac24b4e75e5bb06cec683e802e31f52602a2263d1")
                .unwrap()
        );

        let deposit1 = TransactionData::new(alice_address, contract.address(), 10)
            .signed(alice_secret, &mut rng);
        let deposit2 =
            TransactionData::new(bob_address, contract.address(), 0).signed(bob_secret, &mut rng);
        contract.deposit(deposit1);
        contract.deposit(deposit2);

        let pending_deposits = contract.deposits();
        assert_eq!(pending_deposits.len(), 2);
        operator.process_deposits(pending_deposits.clone(), &poseidon);

        let root_after_dep = operator.state_root();

        assert_eq!(
            root_after_dep,
            Fp::from_hex("0x088ae6b72631cee61d28ca13634796e8a90e2703604356aafcaea984e317d1f4")
                .unwrap()
        );

        // should periodically check deposits on the contract

        // assert!(contract.check_balance(MerkleProof::default()) == alice.balance());
        // assert!(contract.check_balance(MerkleProof::default()) == bob.balance()));

        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        assert!(operator.execute_transaction(t1, &poseidon).is_none());
        let batch = operator.execute_transaction(t2, &poseidon).unwrap();
        let root_after_tx = operator.state_root();
        assert_eq!(
            root_after_tx,
            Fp::from_hex("0x088ae6b72631cee61d28ca13634796e8a90e2703604356aafcaea984e317d1f4")
                .unwrap()
        );

        let txs = batch.transactions();
        let expected_txs = vec![t1, t2];
        assert_eq!(txs, &expected_txs);
        assert_eq!(batch.roots(), (root_after_tx, root_after_tx));

        // Should send a proof to the contract inside and update state on chain

        // check calldata on chain
    }
}
