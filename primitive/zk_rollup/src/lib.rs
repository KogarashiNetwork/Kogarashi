#![allow(dead_code)]
#![allow(unused_variables)]
mod main_contract;
mod merkle_tree;
mod operator;
mod poseidon;
mod proof;
mod redjubjub_circuit;
mod verifier_contract;

#[cfg(test)]
mod tests {

    use std::vec;

    use jub_jub::{Fp, JubjubExtended};
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{PublicKey, SecretKey};
    use zkstd::common::{CurveGroup, Group};

    use crate::{
        main_contract::MainContract,
        operator::{RollupOperator, TransactionData},
        poseidon::Poseidon,
    };

    #[test]
    fn test_deposit_and_transfer_works() {
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut contract =
            MainContract::<Fp>::new(PublicKey::new(JubjubExtended::random(&mut rng)));
        let mut operator = RollupOperator::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT>::default();
        let poseidon = Poseidon::<Fp, 2>::new();
        const ACCOUNT_LIMIT: usize = 2;

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();

        let root_before_dep = operator.state_root();
        assert_eq!(root_before_dep, Fp::zero());

        let deposit1 = TransactionData::new(alice_address, contract.address(), 10)
            .signed(alice_secret, &mut rng);
        let deposit2 =
            TransactionData::new(bob_address, contract.address(), 0).signed(bob_secret, &mut rng);
        contract.deposit(deposit1);
        contract.deposit(deposit2);

        let pending_deposits = contract.deposits();
        assert_eq!(pending_deposits.len(), ACCOUNT_LIMIT);
        operator.process_deposits(pending_deposits.clone(), &poseidon);

        let root_after_dep = operator.state_root();

        assert_eq!(
            root_after_dep,
            Fp::from_hex("0x0b110370e1cda66a7105997fe1196ffe2a685a8d6ccba7aad79e21823cc163c0")
                .unwrap()
        );

        // should periodically check deposits on the contract

        // assert!(contract.check_balance(MerkleProof::default()) == alice.balance());
        // assert!(contract.check_balance(MerkleProof::default()) == bob.balance());)

        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        assert!(operator.execute_transaction(t1, &poseidon).is_none());
        let batch = operator.execute_transaction(t2, &poseidon).unwrap();
        let root_after_tx = operator.state_root();
        assert_eq!(
            root_after_tx,
            Fp::from_hex("0x0b110370e1cda66a7105997fe1196ffe2a685a8d6ccba7aad79e21823cc163c0")
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
