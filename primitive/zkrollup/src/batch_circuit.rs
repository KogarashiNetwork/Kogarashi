mod merkle;

use zero_plonk::prelude::*;
use zkstd::common::{Pairing, SigUtils};

use crate::{
    domain::{RollupTransactionInfo, Transaction, UserData},
    operator::Batch,
    redjubjub_circuit::check_signature,
    FieldHasher,
};
use red_jubjub::sapling_hash;

use self::merkle::check_membership;

#[derive(Debug, PartialEq, Default)]
pub struct BatchCircuit<
    P: Pairing,
    H: FieldHasher<P::ScalarField, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    batch: Batch<P, H, N, BATCH_SIZE>,
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize>
    BatchCircuit<P, H, N, BATCH_SIZE>
{
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(batch: Batch<P, H, N, BATCH_SIZE>) -> Self {
        Self { batch }
    }
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize>
    Circuit<P> for BatchCircuit<P, H, N, BATCH_SIZE>
{
    fn circuit(&self, composer: &mut Builder<P>) -> Result<(), Error> {
        for RollupTransactionInfo {
            transaction,
            pre_root,
            post_root,
            pre_sender,
            pre_receiver,
            pre_sender_proof,
            pre_receiver_proof,
            post_sender_proof,
            post_receiver_proof,
        } in self.batch.transactions.iter()
        {
            let Transaction(sig, t) = transaction;

            check_membership(
                composer,
                pre_sender.to_field_element(),
                *pre_root,
                pre_sender_proof.path,
                pre_sender_proof.path_pos,
            )?;

            check_membership(
                composer,
                pre_receiver.to_field_element(),
                *pre_root,
                pre_receiver_proof.path,
                pre_receiver_proof.path_pos,
            )?;

            check_signature(
                composer,
                t.sender_address.inner().into(),
                *sig,
                sapling_hash(&sig.r(), &t.sender_address.to_bytes(), &t.to_bytes()),
            )?;

            let post_sender = UserData {
                balance: pre_sender.balance - t.amount,
                ..*pre_sender
            };

            let post_receiver = UserData {
                balance: pre_receiver.balance + t.amount,
                ..*pre_receiver
            };

            check_membership(
                composer,
                post_sender.to_field_element(),
                *post_root,
                post_sender_proof.path,
                post_sender_proof.path_pos,
            )?;

            check_membership(
                composer,
                post_receiver.to_field_element(),
                *post_root,
                post_receiver_proof.path,
                post_receiver_proof.path_pos,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use bls_12_381::Fr;
    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use poly_commit::KzgParams;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{PublicKey, SecretKey};
    use zero_plonk::prelude::*;
    use zkstd::common::{CurveGroup, Group};

    use crate::{
        domain::{TransactionData, UserData},
        operator::RollupOperator,
        poseidon::Poseidon,
    };

    use super::BatchCircuit;

    #[test]
    fn batch_circuit_test() {
        let n = 15;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = KzgParams::setup(n, BlsScalar::random(&mut rng));

        const ACCOUNT_LIMIT: usize = 2;
        const BATCH_SIZE: usize = 2;
        // Create an operator and contract
        let mut operator =
            RollupOperator::<TatePairing, Poseidon<Fr, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
                Poseidon::<Fr, 2>::new(),
                pp.clone(),
            );
        let contract_address = PublicKey::new(JubjubExtended::random(&mut rng));

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();

        let alice = UserData::new(0, 10, alice_address);
        let bob = UserData::new(1, 0, bob_address);

        let deposit1 = TransactionData::new(alice_address, contract_address, 10)
            .signed(alice_secret, &mut rng);
        let deposit2 =
            TransactionData::new(bob_address, contract_address, 0).signed(bob_secret, &mut rng);

        // Explicitly process data on L2. Will be changed, when communication between layers will be decided.
        operator.process_deposit(deposit1);
        operator.process_deposit(deposit2);

        // Prepared and sign transfer transactions
        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        assert!(operator.execute_transaction(t1).is_none());
        let (proof, batch) = operator.execute_transaction(t2).unwrap();

        let batch_circuit = BatchCircuit::new(batch);

        let prover = Compiler::compile::<
            BatchCircuit<TatePairing, Poseidon<Fr, 2>, ACCOUNT_LIMIT, BATCH_SIZE>,
            TatePairing,
        >(&mut pp, label)
        .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &batch_circuit)
            .expect("failed to prove");
    }
}