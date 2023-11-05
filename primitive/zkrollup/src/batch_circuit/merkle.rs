use bls_12_381::Fr;
use zkplonk::prelude::*;
use zksnarks::{plonk::wire::PrivateWire, Constraint};
use zkstd::common::{vec, Group, TwistedEdwardsAffine, Vec};

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

fn hash<C: TwistedEdwardsAffine>(
    composer: &mut Plonk<C>,
    inputs: (PrivateWire, PrivateWire),
) -> PrivateWire {
    let sum = Constraint::default()
        .left(1)
        .constant(C::Range::ADDITIVE_GENERATOR)
        .a(inputs.0);
    let gen_plus_first = composer.gate_add(sum);

    let first_hash = Constraint::default().left(42).a(gen_plus_first);
    let first_hash = composer.gate_add(first_hash);

    let sum = Constraint::default()
        .left(1)
        .constant(C::Range::ADDITIVE_GENERATOR)
        .a(inputs.1);

    let gen_plus_second = composer.gate_add(sum);

    let sum_plus_42 = Constraint::default()
        .left(1)
        .constant(C::Range::from(42))
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

fn calculate_root<C: TwistedEdwardsAffine, const N: usize>(
    composer: &mut Plonk<C>,
    leaf: C::Range,
    path: &[(C::Range, C::Range)],
    path_pos: &[u64],
) -> Result<PrivateWire, Error> {
    let mut prev = composer.append_witness(leaf);

    let path: Vec<(PrivateWire, PrivateWire)> = path
        .iter()
        .map(|(node_l, node_r)| {
            (
                composer.append_witness(*node_l),
                composer.append_witness(*node_r),
            )
        })
        .collect();

    let path_pos: Vec<PrivateWire> = path_pos
        .iter()
        .map(|pos| composer.append_witness(C::Range::from(*pos)))
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

        prev = hash::<C>(composer, (left, right));
    }

    Ok(prev)
}

pub(crate) fn check_membership<C: TwistedEdwardsAffine, const N: usize>(
    composer: &mut Plonk<C>,
    leaf: C::Range,
    root: C::Range,
    path: &[(C::Range, C::Range)],
    path_pos: &[u64],
) -> Result<(), Error> {
    assert_eq!(path.len(), path_pos.len());

    let precomputed_root = composer.append_witness(root);

    let root = calculate_root::<C, N>(composer, leaf, path, path_pos)?;
    composer.assert_equal(root, precomputed_root);
    Ok(())
}

impl<const N: usize> Circuit<JubjubAffine> for MerkleMembershipCircuit<N> {
    type ConstraintSystem = Plonk<JubjubAffine>;
    fn synthesize(&self, composer: &mut Plonk<JubjubAffine>) -> Result<(), Error> {
        check_membership::<JubjubAffine, N>(
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
    use super::MerkleMembershipCircuit;
    use crate::{domain::UserData, merkle_tree::SparseMerkleTree, poseidon::Poseidon};

    use bls_12_381::Fr;
    use ec_pairing::TatePairing;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{PublicKey, RedJubjub};
    use zkplonk::prelude::*;
    use zksnarks::keypair::Keypair;
    use zksnarks::plonk::PlonkParams;
    use zksnarks::public_params::PublicParameters;
    use zkstd::common::TwistedEdwardsCurve;

    #[test]
    fn merkle_check_membership() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);
        let mut pp = PlonkParams::setup(n, &mut rng);

        let (prover, verifier) =
            PlonkKey::<TatePairing, JubjubAffine, MerkleMembershipCircuit<3>>::compile(&mut pp)
                .expect("failed to compile circuit");

        let poseidon = Poseidon::<Fr, 2>::new();

        let mut merkle_tree =
            SparseMerkleTree::<Fr, Poseidon<Fr, 2>, 3>::new_empty(&poseidon, &[0; 64]).unwrap();

        // Sibling hashes before update
        let merkle_proof = merkle_tree.generate_membership_proof(0);

        // New leaf data
        let user =
            UserData::<RedJubjub>::new(0, 10, PublicKey::new(JubjubExtended::random(&mut rng)));

        let merkle_circuit = MerkleMembershipCircuit::<3>::new(
            user.to_field_element(),
            merkle_tree.root(),
            merkle_proof.path,
            merkle_proof.path_pos,
        );

        // Should fail
        assert!(prover.create_proof(&mut rng, &merkle_circuit).is_err());

        merkle_tree
            .update(0, user.to_field_element(), &poseidon)
            .unwrap();

        let merkle_proof = merkle_tree.generate_membership_proof(0);

        let merkle_circuit = MerkleMembershipCircuit::<3>::new(
            user.to_field_element(),
            merkle_tree.root(),
            merkle_proof.path,
            merkle_proof.path_pos,
        );

        let (proof, public_inputs) = prover
            .create_proof(&mut rng, &merkle_circuit)
            .expect("failed to prove");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
    }
}
