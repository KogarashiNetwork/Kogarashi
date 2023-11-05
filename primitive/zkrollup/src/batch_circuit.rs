mod merkle;
use core::fmt::Debug;

use zkplonk::prelude::*;
use zkstd::common::{RedDSA, SigUtils};

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
    P: RedDSA,
    H: FieldHasher<P::Range, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    batch: Batch<P, H, N, BATCH_SIZE>,
}

impl<P: RedDSA, H: FieldHasher<P::Range, 2>, const N: usize, const BATCH_SIZE: usize>
    BatchCircuit<P, H, N, BATCH_SIZE>
{
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(batch: Batch<P, H, N, BATCH_SIZE>) -> Self {
        Self { batch }
    }
}

impl<
        P: RedDSA + Debug + Default,
        H: FieldHasher<P::Range, 2>,
        const N: usize,
        const BATCH_SIZE: usize,
    > Circuit<P::Affine> for BatchCircuit<P, H, N, BATCH_SIZE>
{
    type ConstraintSystem = Plonk<P::Affine>;
    fn synthesize(&self, composer: &mut Plonk<P::Affine>) -> Result<(), Error> {
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
            is_withdrawal,
        } in self.batch.transactions.iter()
        {
            let Transaction(sig, t) = transaction;

            check_membership::<P::Affine, N>(
                composer,
                pre_sender.to_field_element(),
                *pre_root,
                &pre_sender_proof.path,
                &pre_sender_proof.path_pos,
            )?;

            check_membership::<P::Affine, N>(
                composer,
                pre_receiver.to_field_element(),
                *pre_root,
                &pre_receiver_proof.path,
                &pre_receiver_proof.path_pos,
            )?;

            check_signature::<P>(
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

            check_membership::<P::Affine, N>(
                composer,
                post_sender.to_field_element(),
                *post_root,
                &post_sender_proof.path,
                &post_sender_proof.path_pos,
            )?;

            check_membership::<P::Affine, N>(
                composer,
                post_receiver.to_field_element(),
                *post_root,
                &post_receiver_proof.path,
                &post_receiver_proof.path_pos,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BatchCircuit;
    use crate::{
        domain::{TransactionData, UserData},
        operator::RollupOperator,
        poseidon::Poseidon,
    };

    use bls_12_381::Fr;
    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{RedJubjub, SecretKey};
    use zkplonk::prelude::*;
    use zksnarks::keypair::Keypair;
    use zksnarks::plonk::PlonkParams;
    use zksnarks::public_params::PublicParameters;
    use zkstd::common::Group;

    #[test]
    fn batch_circuit_test() {
        let n = 15;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = PlonkParams::setup(n, &mut rng);

        const ACCOUNT_LIMIT: usize = 3;
        const BATCH_SIZE: usize = 2;
        // Create an operator and contract
        let mut operator = RollupOperator::<
            RedJubjub,
            TatePairing,
            Poseidon<Fr, 2>,
            ACCOUNT_LIMIT,
            BATCH_SIZE,
        >::new(Poseidon::<Fr, 2>::new(), pp.clone());

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();

        let alice = UserData::new(0, 10, alice_address);
        let bob = UserData::new(1, 0, bob_address);

        // Explicitly process data on L2. Will be changed, when communication between layers will be decided.
        operator.process_deposit(10, alice_address);
        operator.process_deposit(0, bob_address);

        // Prepared and sign transfer transactions
        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        assert!(operator.execute_transaction(t1).is_none());
        let ((proof, public_inputs), batch) = operator.execute_transaction(t2).unwrap();

        let (_, verifier) = PlonkKey::<
            TatePairing,
            JubjubAffine,
            BatchCircuit<RedJubjub, Poseidon<Fr, 2>, ACCOUNT_LIMIT, BATCH_SIZE>,
        >::compile(&mut pp)
        .expect("failed to compile circuit");

        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
    }
}
