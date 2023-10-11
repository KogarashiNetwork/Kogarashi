use bls_12_381::Fr;
use ec_pairing::TatePairing;
use zero_plonk::prelude::*;
use zksnarks::{plonk::wire::Wire, Constraint};
use zkstd::behave::Group;
use zkstd::common::{vec, Pairing, Vec};

#[derive(Debug, PartialEq)]
pub struct MerkleMembershipCircuit<const N: usize> {
    leaf: Fr,
    root: Fr,
    path: Vec<(Fr, Fr)>,
    path_pos: Vec<u64>,
}

impl<const N: usize> Default for MerkleMembershipCircuit<N> {
    fn default() -> Self {
        Self {
            leaf: Default::default(),
            path: vec![(Fr::zero(), Fr::zero()); N - 1],
            root: Default::default(),
            path_pos: vec![0; N - 1],
        }
    }
}

impl<const N: usize> MerkleMembershipCircuit<N> {
    pub(crate) fn new(leaf: Fr, root: Fr, path: Vec<(Fr, Fr)>, path_pos: Vec<u64>) -> Self {
        Self {
            leaf,
            path,
            root,
            path_pos,
        }
    }
}

fn hash<P: Pairing>(composer: &mut Builder<P>, inputs: (Wire, Wire)) -> Wire {
    let sum = Constraint::default()
        .left(1)
        .constant(P::ScalarField::ADDITIVE_GENERATOR)
        .a(inputs.0);
    let gen_plus_first = composer.gate_add(sum);

    let first_hash = Constraint::default().left(42).a(gen_plus_first);
    let first_hash = composer.gate_add(first_hash);

    let sum = Constraint::default()
        .left(1)
        .constant(P::ScalarField::ADDITIVE_GENERATOR)
        .a(inputs.1);

    let gen_plus_second = composer.gate_add(sum);

    let sum_plus_42 = Constraint::default()
        .left(1)
        .constant(P::ScalarField::from(42))
        .a(first_hash);
    let sum_plus_42 = composer.gate_add(sum_plus_42);

    let second_hash = Constraint::default()
        .mult(1)
        .a(gen_plus_second)
        .b(sum_plus_42);
    let second_hash = composer.gate_mul(second_hash);

    composer.gate_add(
        Constraint::default()
            .left(1)
            .right(1)
            .a(first_hash)
            .b(second_hash),
    )
}

fn calculate_root<P: Pairing, const N: usize>(
    composer: &mut Builder<P>,
    leaf: P::ScalarField,
    path: &[(P::ScalarField, P::ScalarField)],
    path_pos: &[u64],
) -> Result<Wire, Error> {
    let mut prev = composer.append_witness(leaf);

    let path: Vec<(Wire, Wire)> = path
        .iter()
        .map(|(node_l, node_r)| {
            (
                composer.append_witness(*node_l),
                composer.append_witness(*node_r),
            )
        })
        .collect();

    let path_pos: Vec<Wire> = path_pos
        .iter()
        .map(|pos| composer.append_witness(P::JubjubScalar::from(*pos)))
        .collect();

    for ((left, right), pos) in path.into_iter().zip(path_pos) {
        // TODO: If provided leaf == 0, and w1 == 0 || w2 == 0, then we pass both checks with invalid leaf
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

pub(crate) fn check_membership<P: Pairing, const N: usize>(
    composer: &mut Builder<P>,
    leaf: P::ScalarField,
    root: P::ScalarField,
    path: &[(P::ScalarField, P::ScalarField)],
    path_pos: &[u64],
) -> Result<(), Error> {
    assert_eq!(path.len(), path_pos.len());

    let precomputed_root = composer.append_witness(root);

    let root = calculate_root::<P, N>(composer, leaf, path, path_pos)?;
    composer.assert_equal(root, precomputed_root);
    Ok(())
}

impl<const N: usize> Circuit<TatePairing> for MerkleMembershipCircuit<N> {
    fn circuit(&self, composer: &mut Builder<TatePairing>) -> Result<(), Error> {
        check_membership::<TatePairing, N>(
            composer,
            self.leaf,
            self.root,
            &self.path,
            &self.path_pos,
        )
    }
}

#[cfg(test)]
mod tests {

    use bls_12_381::Fr;
    use ec_pairing::TatePairing;
    use poly_commit::PublicParameters;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::PublicKey;
    use zero_plonk::prelude::*;
    use zksnarks::plonk::PlonkParams;
    use zkstd::common::{CurveGroup, Group};

    use crate::{domain::UserData, merkle_tree::SparseMerkleTree, poseidon::Poseidon};

    use super::MerkleMembershipCircuit;

    #[test]
    fn merkle_check_membership() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = PlonkParams::setup(n, BlsScalar::random(&mut rng));

        let (prover, verifier) =
            Compiler::compile::<MerkleMembershipCircuit<3>, TatePairing>(&mut pp, label)
                .expect("failed to compile circuit");

        let poseidon = Poseidon::<Fr, 2>::new();

        let mut merkle_tree =
            SparseMerkleTree::<Fr, Poseidon<Fr, 2>, 3>::new_empty(&poseidon, &[0; 64]).unwrap();

        // Sibling hashes before update
        let merkle_proof = merkle_tree.generate_membership_proof(0);

        // New leaf data
        let user =
            UserData::<TatePairing>::new(0, 10, PublicKey::new(JubjubExtended::random(&mut rng)));

        let merkle_circuit = MerkleMembershipCircuit::new(
            user.to_field_element(),
            merkle_tree.root(),
            merkle_proof.path,
            merkle_proof.path_pos,
        );

        // Should fail
        assert!(prover.prove(&mut rng, &merkle_circuit).is_err());

        merkle_tree
            .update(0, user.to_field_element(), &poseidon)
            .unwrap();

        let merkle_proof = merkle_tree.generate_membership_proof(0);

        let merkle_circuit = MerkleMembershipCircuit::new(
            user.to_field_element(),
            merkle_tree.root(),
            merkle_proof.path,
            merkle_proof.path_pos,
        );

        let (proof, public_inputs) = prover
            .prove(&mut rng, &merkle_circuit)
            .expect("failed to prove");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
    }
}
