use bls_12_381::Fr;
use ec_pairing::TatePairing;
use zero_plonk::prelude::*;
use zksnarks::Witness;
use zkstd::behave::Group;

#[derive(Debug, PartialEq)]
pub struct RootCalculateCircuit<const K: usize> {
    leaf: Fr,
    final_root: Fr,
    path: [(Fr, Fr); K],
    path_pos: [u64; K],
}

impl<const K: usize> Default for RootCalculateCircuit<K> {
    fn default() -> Self {
        Self {
            leaf: Default::default(),
            path: [(Fr::zero(), Fr::zero()); K],
            final_root: Default::default(),
            path_pos: [0; K],
        }
    }
}

impl<const K: usize> RootCalculateCircuit<K> {
    pub(crate) fn new(leaf: Fr, final_root: Fr, path: [(Fr, Fr); K], path_pos: [u64; K]) -> Self {
        Self {
            leaf,
            path,
            final_root,
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

impl<const K: usize> Circuit<TatePairing> for RootCalculateCircuit<K> {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer<TatePairing>,
    {
        let final_root = composer.append_witness(self.final_root);
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
            let left = composer.component_select(pos, left, prev);
            let right = composer.component_select(pos, prev, right);

            prev = hash(composer, (left, right));
        }

        composer.assert_equal(prev, final_root);

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

    use super::RootCalculateCircuit;

    #[test]
    fn merkle_root_update() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = KeyPair::setup(n, BlsScalar::random(&mut rng));

        let poseidon = Poseidon::<Fr, 2>::new();

        let mut merkle_tree =
            SparseMerkleTree::<Fr, Poseidon<Fr, 2>, 1>::new_empty(&poseidon, &[0; 64]).unwrap();

        // Sibling hashes before update
        let proof = merkle_tree.generate_membership_proof(0);

        // New leaf data
        let user = UserData::new(0, 10, PublicKey::new(JubjubExtended::random(&mut rng)));

        merkle_tree
            .update(0, user.to_field_element(), &poseidon)
            .unwrap();
        let final_root = merkle_tree.root();

        let merkle_circuit = RootCalculateCircuit::new(
            user.to_field_element(),
            final_root,
            proof.path,
            proof.path_pos,
        );

        let prover = Compiler::compile::<RootCalculateCircuit<1>, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &merkle_circuit)
            .expect("failed to prove");
    }
}
