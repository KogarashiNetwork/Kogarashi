#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod batch_circuit;
mod db;
mod domain;
mod main_contract;
mod merkle_tree;
mod operator;
mod poseidon;
mod proof;
mod redjubjub_circuit;
mod verifier_contract;

pub use poseidon::FieldHasher;

#[cfg(test)]
mod tests {

    use jub_jub::{Fp, JubjubExtended};
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{PublicKey, SecretKey};
    use zkstd::common::{vec, CurveGroup, Group, Vec};

    use crate::{
        domain::{Transaction, TransactionData},
        main_contract::MainContract,
        operator::RollupOperator,
        poseidon::Poseidon,
    };

    #[test]
    fn test_zkrollup() {
        let mut rng = StdRng::seed_from_u64(8349u64);
        const ACCOUNT_LIMIT: usize = 2;
        const BATCH_SIZE: usize = 2;

        // 1. Create an operator and contract
        let mut operator = RollupOperator::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
            Poseidon::<Fp, 2>::new(),
        );
        let mut contract = MainContract::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
            operator.state_root(),
            PublicKey::new(JubjubExtended::random(&mut rng)),
        );

        // Assures that null elements' hashes are correct
        let root_before_dep = operator.state_root();
        assert_eq!(
            root_before_dep,
            Fp::from_hex("0x082e6d1a102e14de34bf3471c6a79c4ae3069fbaad7346032d40626576cf4039")
                .unwrap()
        );

        // 2. Generate user data
        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();
        // Decided by the operator
        let withdraw_address = PublicKey::new(JubjubExtended::random(&mut rng));
        // Will be changed. Implementation for test
        operator.add_withdrawal_address(withdraw_address);
        // State root will be changed here, but we can ignore it.

        // 3. Create and sign deposit transactions
        let deposit1 = TransactionData::new(alice_address, contract.address(), 10)
            .signed(alice_secret, &mut rng);
        let deposit2 =
            TransactionData::new(bob_address, contract.address(), 0).signed(bob_secret, &mut rng);

        // 4. Add them to the deposit pool on the L1
        contract.deposit(deposit1);
        contract.deposit(deposit2);

        let pending_deposits = contract.deposits();
        assert_eq!(pending_deposits.len(), 2);
        // 5. Explicitly process data on L2. Will be changed, when communication between layers will be decided.
        operator.process_deposits(pending_deposits.clone());

        // Assures that deposits were processed on L2 and state tree was changed.
        let root_after_dep = operator.state_root();
        assert_eq!(
            root_after_dep,
            Fp::from_hex("0x0e19d7c5c79887947f8f9e73f07570eaabc7a4d2f5efb1c34b0b5d40e63ec4d1")
                .unwrap()
        );

        // Need to implement balance verification for users through the contract
        // assert!(contract.check_balance(MerkleProof::default()) == alice.balance());
        // assert!(contract.check_balance(MerkleProof::default()) == bob.balance()));

        // 6. Prepared and sign transfer transactions
        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        // 7. Execute transactions on L2
        assert!(operator.execute_transaction(t1).is_none());
        // With BATCH_SIZE == 2 second transaction should create a proof and batch
        let (proof, batch) = operator.execute_transaction(t2).unwrap();
        let root_after_tx = operator.state_root();
        // State root should change as wells
        assert_eq!(
            root_after_tx,
            Fp::from_hex("0x0e19d7c5c79887947f8f9e73f07570eaabc7a4d2f5efb1c34b0b5d40e63ec4d1")
                .unwrap()
        );

        // 8. Explicitly add_batch on L1. Will be changed, when communication between layers will be decided.
        contract.add_batch(proof, batch);

        // 9. Check that batch info is on L1.
        assert_eq!(contract.calldata.len(), 1);
        let batch = contract.calldata.first().unwrap();
        let txs: Vec<Transaction> = batch.raw_transactions().cloned().collect();
        let expected_txs = vec![t1, t2];
        assert_eq!(&txs, &expected_txs);
        assert_eq!(batch.border_roots(), (root_after_dep, root_after_tx));
        // 10. Check that state root on L1 changed.
        assert_eq!(contract.rollup_state_root, root_after_tx);

        // Withdrawal

        // 1. Burn funds on L2 by sending to a special address
        let alice_withdraw: Transaction =
            TransactionData::new(alice_address, withdraw_address, 5).signed(alice_secret, &mut rng);
        let bob_withdraw: Transaction =
            TransactionData::new(bob_address, withdraw_address, 5).signed(bob_secret, &mut rng);

        operator.execute_transaction(alice_withdraw);
        let (proof, batch) = operator.execute_transaction(bob_withdraw).unwrap();

        // 2. l2_burn_merkle_proof_alice and l2_burn_merkle_proof_bob should be generated with batch_tree
        // Will decide the process, while implementing the gadget

        // 3. Withdraw on L1
        contract.withdraw(
            // l2_burn_merkle_proof_alice,
            batch.final_root(),
            alice_withdraw,
            alice_address,
        );

        contract.withdraw(
            // l2_burn_merkle_proof_bob,
            batch.final_root(),
            bob_withdraw,
            bob_address,
        );
    }
}
