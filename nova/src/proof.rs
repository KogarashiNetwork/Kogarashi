use crate::{
    relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness},
    RelaxedR1cs,
};

use r1cs::{CircuitDriver, DenseVectors, R1cs};
use zkstd::common::{Group, Ring};

pub struct RecursiveProof<C: CircuitDriver> {
    i: usize,
    z0: DenseVectors<C::Scalar>,
    zi: DenseVectors<C::Scalar>,
    r1cs: R1cs<C>,
    pair: (
        (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>),
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
        } = self.clone();
        let ((l_ui, l_wi), (s_ui, s_wi)) = pair;

        if *i == 0 {
            z0 == zi
        } else {
            // check if folded instance has default error vectors and scalar
            let is_folded_instance_default =
                s_ui.commit_e == C::Affine::ADDITIVE_IDENTITY && s_ui.u == C::Scalar::one();

            // check if instance-witness pair satisfy
            let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
            let l_relaxed_r1cs = relaxed_r1cs.update(&l_ui, &l_wi);
            let s_relaxed_r1cs = relaxed_r1cs.update(&s_ui, &s_wi);
            let is_instance_witness_sat = l_relaxed_r1cs.is_sat() && s_relaxed_r1cs.is_sat();

            is_folded_instance_default && is_instance_witness_sat
        }
    }
}
