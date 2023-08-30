use ec_pairing::TatePairing;
use zero_plonk::prelude::*;
use zksnarks::Witness;
use zkstd::behave::Group;
#[derive(Debug, PartialEq)]
pub struct RootCalculateCircuit<const K: usize> {
    leaf: JubjubScalar,
    final_root: JubjubScalar,
    path: [(JubjubScalar, JubjubScalar); K],
    path_pos: [u64; K],
}

impl<const K: usize> Default for RootCalculateCircuit<K> {
    fn default() -> Self {
        Self {
            leaf: Default::default(),
            path: [(JubjubScalar::zero(), JubjubScalar::zero()); K],
            final_root: Default::default(),
            path_pos: [0; K],
        }
    }
}

impl<const K: usize> RootCalculateCircuit<K> {
    pub(crate) fn new(
        leaf: JubjubScalar,
        final_root: JubjubScalar,
        path: [(JubjubScalar, JubjubScalar); K],
        path_pos: [u64; K],
    ) -> Self {
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
        .constant(JubjubScalar::ADDITIVE_GENERATOR)
        .a(inputs.0);
    let gen_plus_first = composer.gate_add(sum);

    let first_hash = Constraint::new()
        .mult(1)
        .a(gen_plus_first)
        .constant(JubjubScalar::from(2_u64));
    let first_hash = composer.gate_add(first_hash);

    let sum = Constraint::new()
        .left(1)
        .constant(JubjubScalar::ADDITIVE_GENERATOR)
        .a(inputs.1);
    let gen_plus_second = composer.gate_add(sum);

    let second_hash = Constraint::new()
        .mult(1)
        .a(gen_plus_second)
        .constant(JubjubScalar::from(2_u64));
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
            let cur = composer.component_select(pos, right, left);
            composer.assert_equal(prev, cur);
            prev = hash(composer, (left, right));
        }

        composer.assert_equal(prev, final_root);

        Ok(())
    }
}
