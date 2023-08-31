mod root;

use ec_pairing::TatePairing;
use zero_plonk::prelude::*;

use crate::{domain::Transaction, operator::Batch};

#[derive(Debug, PartialEq, Default)]
pub struct BatchCircuit {
    batch: Batch<JubjubScalar>,
    initial_root: JubjubScalar,
    final_root: JubjubScalar,
}

impl BatchCircuit {
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        batch: Batch<JubjubScalar>,
        initial_root: JubjubScalar,
        final_root: JubjubScalar,
    ) -> Self {
        Self {
            batch,
            initial_root,
            final_root,
        }
    }
}

impl Circuit<TatePairing> for BatchCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer<TatePairing>,
    {
        let initial_root = composer.append_witness(self.initial_root);
        let final_root = composer.append_witness(self.final_root);
        for t in self.batch.transactions() {
            let Transaction(signature, data) = t;
            // verify_signature
            let sender = data.sender_address;
            let receiver = data.receiver_address;
        }

        // calculate root

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use poly_commit::KeyPair;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::SecretKey;
    use zero_plonk::prelude::*;
    use zkstd::common::Group;

    use crate::{
        domain::{Transaction, TransactionData, UserData},
        merkle_tree::SparseMerkleTree,
        operator::Batch,
        poseidon::Poseidon,
    };

    use super::BatchCircuit;

    fn create_batch(
        txs: (Transaction, Transaction),
        sender: &mut UserData,
        receiver: &mut UserData,
        merkle: &mut SparseMerkleTree<Fp, Poseidon<Fp, 2>, 2>,
    ) -> Batch<Fp> {
        todo!()
    }

    #[test]
    fn batch_update() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = KeyPair::setup(n, BlsScalar::random(&mut rng));

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();

        let mut alice = UserData::new(0, 10, alice_address);
        let mut bob = UserData::new(1, 0, bob_address);

        let poseidon = Poseidon::<Fp, 2>::new();

        let mut merkle_tree = SparseMerkleTree::<Fp, Poseidon<Fp, 2>, 2>::new_sequential(
            &[alice.to_field_element(), bob.to_field_element()],
            &poseidon,
            &[0; 64],
        )
        .unwrap();

        let t1 =
            TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
        let t2 = TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

        let batch = create_batch((t1, t2), &mut alice, &mut bob, &mut merkle_tree);

        let batch_circuit = BatchCircuit::new(batch, Fp::zero(), Fp::zero());

        let prover = Compiler::compile::<BatchCircuit, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &batch_circuit)
            .expect("failed to prove");
    }
}
