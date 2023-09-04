use bls_12_381::Fr;
use ec_pairing::TatePairing;
use zero_plonk::prelude::*;
use zksnarks::Witness;
use zkstd::behave::Group;

#[derive(Debug, PartialEq)]
pub struct MerkleMembershipCircuit<const K: usize> {
    leaf: Fr,
    root: Fr,
    path: [(Fr, Fr); K],
    path_pos: [u64; K],
}

impl<const K: usize> Default for MerkleMembershipCircuit<K> {
    fn default() -> Self {
        Self {
            leaf: Default::default(),
            path: [(Fr::zero(), Fr::zero()); K],
            root: Default::default(),
            path_pos: [0; K],
        }
    }
}

impl<const K: usize> MerkleMembershipCircuit<K> {
    pub(crate) fn new(leaf: Fr, root: Fr, path: [(Fr, Fr); K], path_pos: [u64; K]) -> Self {
        Self {
            leaf,
            path,
            root,
            path_pos,
        }
    }
}

fn hash<C>(composer: &mut C, inputs: (Witness, Witness)) -> Witness
where
    C: Composer<TatePairing>,
{
    let sum: Constraint<TatePairing> = Constraint::new()
        .left(1)
        .constant(Fr::ADDITIVE_GENERATOR)
        .a(inputs.0);
    let gen_plus_first = composer.gate_add(sum);

    let first_hash = Constraint::new().left(2).a(gen_plus_first);
    let first_hash = composer.gate_add(first_hash);

    let sum = Constraint::new()
        .left(1)
        .constant(Fr::ADDITIVE_GENERATOR)
        .a(inputs.1);

    let gen_plus_second = composer.gate_add(sum);

    let second_hash = Constraint::new().left(2).a(gen_plus_second);
    let second_hash = composer.gate_add(second_hash);

    composer.gate_add(
        Constraint::new()
            .left(1)
            .right(1)
            .a(first_hash)
            .b(second_hash),
    )
}

impl<const K: usize> MerkleMembershipCircuit<K> {
    fn calculate_root<C>(&self, composer: &mut C) -> Result<Witness, Error>
    where
        C: Composer<TatePairing>,
    {
        let mut prev = composer.append_witness(self.leaf);

        let path: Vec<(Witness, Witness)> = self
            .path
            .iter()
            .map(|(node_l, node_r)| {
                (
                    composer.append_witness(*node_l),
                    composer.append_witness(*node_r),
                )
            })
            .collect();

        let path_pos: Vec<Witness> = self
            .path_pos
            .iter()
            .map(|pos| composer.append_witness(JubjubScalar::from(*pos)))
            .collect();

        for ((left, right), pos) in path.into_iter().zip(path_pos) {
            // left ^ prev == 0, if equal
            let w1 = composer.append_logic_xor(left, prev, 256);
            // right ^ prev == 0, if equal
            let w2 = composer.append_logic_xor(right, prev, 256);
            // if one is 0, then and will result to 0
            let check = composer.append_logic_and(w1, w2, 256);
            composer.assert_equal_constant(check, 0, None);

            prev = hash(composer, (left, right));
        }

        Ok(prev)
    }
}

impl<const K: usize> Circuit<TatePairing> for MerkleMembershipCircuit<K> {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer<TatePairing>,
    {
        let real_root = composer.append_witness(self.root);
        let root = self.calculate_root(composer)?;
        composer.assert_equal(root, real_root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use bls_12_381::Fr;
    use ec_pairing::TatePairing;
    use poly_commit::KeyPair;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::PublicKey;
    use zero_plonk::prelude::*;
    use zkstd::common::{CurveGroup, Group};

    use crate::{domain::UserData, merkle_tree::SparseMerkleTree, poseidon::Poseidon};

    use super::MerkleMembershipCircuit;

    #[test]
    fn merkle_check_membership() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = KeyPair::setup(n, BlsScalar::random(&mut rng));

        let poseidon = Poseidon::<Fr, 2>::new();

        let mut merkle_tree =
            SparseMerkleTree::<Fr, Poseidon<Fr, 2>, 2>::new_empty(&poseidon, &[0; 64]).unwrap();

        // Sibling hashes before update
        let proof = merkle_tree.generate_membership_proof(0);

        // New leaf data
        let user = UserData::new(0, 10, PublicKey::new(JubjubExtended::random(&mut rng)));

        let merkle_circuit = MerkleMembershipCircuit::new(
            user.to_field_element(),
            merkle_tree.root(),
            proof.path,
            proof.path_pos,
        );

        let prover = Compiler::compile::<MerkleMembershipCircuit<2>, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        // Should fail
        assert!(prover.0.prove(&mut rng, &merkle_circuit).is_err());

        merkle_tree
            .update(0, user.to_field_element(), &poseidon)
            .unwrap();

        let proof = merkle_tree.generate_membership_proof(0);

        let merkle_circuit = MerkleMembershipCircuit::new(
            user.to_field_element(),
            merkle_tree.root(),
            proof.path,
            proof.path_pos,
        );

        let prover = Compiler::compile::<MerkleMembershipCircuit<2>, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &merkle_circuit)
            .expect("failed to prove");
    }
}
