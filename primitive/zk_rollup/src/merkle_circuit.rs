use ec_pairing::TatePairing;
use zero_plonk::prelude::*;

/// Confidential transfer circuit
#[derive(Debug, PartialEq, Default)]
pub struct MerkleCircuit {
    initial_root: JubjubScalar,
    final_root: JubjubScalar,
    leaf: JubjubScalar,
}

impl MerkleCircuit {
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        initial_root: JubjubScalar,
        final_root: JubjubScalar,
        leaf: JubjubScalar,
    ) -> Self {
        Self {
            initial_root,
            final_root,
            leaf,
        }
    }
}

impl Circuit<TatePairing> for MerkleCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer<TatePairing>,
    {
        let initial_root = composer.append_witness(self.initial_root);
        let final_root = composer.append_witness(self.final_root);
        let leaf = composer.append_witness(self.leaf);

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
    use red_jubjub::PublicKey;
    use zero_plonk::prelude::*;
    use zkstd::common::{CurveGroup, Group};

    use crate::{domain::UserData, merkle_tree::SparseMerkleTree, poseidon::Poseidon};

    use super::MerkleCircuit;

    #[test]
    fn merkle_update() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = KeyPair::setup(n, BlsScalar::random(&mut rng));

        let poseidon = Poseidon::<Fp, 2>::new();
        let mut merkle_tree =
            SparseMerkleTree::<Fp, Poseidon<Fp, 2>, 1>::new_empty(&poseidon, &[0; 64]).unwrap();

        let initial_root = merkle_tree.root();
        let user = UserData::new(0, 10, PublicKey::new(JubjubExtended::random(&mut rng)));
        merkle_tree
            .update(0, user.to_field_element(), &poseidon)
            .unwrap();
        let final_root = merkle_tree.root();

        let merkle_circuit = MerkleCircuit::new(initial_root, final_root, user.to_field_element());
        let prover = Compiler::compile::<MerkleCircuit, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &merkle_circuit)
            .expect("failed to prove");
    }
}
