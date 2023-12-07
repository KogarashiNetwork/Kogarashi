use crate::{
    relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness},
    RelaxedR1cs,
};

use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Group, Ring};
use zkstd::matrix::DenseVectors;

#[allow(clippy::type_complexity)]
pub struct RecursiveProof<C: CircuitDriver> {
    pub(crate) i: usize,
    pub(crate) z0: DenseVectors<C::Scalar>,
    pub(crate) zi: DenseVectors<C::Scalar>,
    pub(crate) r1cs: R1cs<C>,
    pub(crate) pair: (
        // instance-witness pair of instance to be folded
        (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>),
        // instance-witness pair of running instance
        (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>),
    ),
}

impl<C: CircuitDriver> RecursiveProof<C> {
    pub fn verify(&self) -> bool {
        let Self {
            i,
            z0,
            zi,
            r1cs,
            pair,
        } = self;
        let ((l_ui, l_wi), (s_ui, s_wi)) = pair;

        if *i == 0 {
            // check if z vector is the same
            z0 == zi
        } else {
            // check that ui.x = hash(vk, i, z0, zi, Ui)
            let expected_x = l_ui.hash(*i, z0, zi);
            let check_hash = expected_x == s_ui.x[0];

            // check if folded instance has default error vectors and scalar
            let check_defaults =
                s_ui.commit_e == C::Affine::ADDITIVE_IDENTITY && s_ui.u == C::Scalar::one();

            // check if instance-witness pair satisfy
            let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
            let l_relaxed_r1cs = relaxed_r1cs.update(l_ui, l_wi);
            let s_relaxed_r1cs = relaxed_r1cs.update(s_ui, s_wi);
            let is_instance_witness_sat = l_relaxed_r1cs.is_sat() && s_relaxed_r1cs.is_sat();

            check_hash && check_defaults && is_instance_witness_sat
        }
    }
}
