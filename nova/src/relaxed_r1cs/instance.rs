use crate::hash::{MimcRO, MIMC_ROUNDS};
use crate::RelaxedR1cs;
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{BNAffine, BNProjective, CurveGroup, Group, PrimeField, Ring};
use zkstd::matrix::DenseVectors;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RelaxedR1csInstance<C: CircuitDriver> {
    /// commitment for witness vectors
    pub(crate) commit_w: C::Affine,
    /// commitment for error vectors
    pub(crate) commit_e: C::Affine,
    /// scalar
    pub(crate) u: C::Scalar,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> RelaxedR1csInstance<C> {
    pub(crate) fn new(x: DenseVectors<C::Scalar>) -> Self {
        Self {
            commit_w: C::Affine::ADDITIVE_IDENTITY,
            commit_e: C::Affine::ADDITIVE_IDENTITY,
            u: C::Scalar::one(),
            x,
        }
    }

    pub(crate) fn fold(&self, r1cs: &RelaxedR1cs<C>, r: C::Scalar, commit_t: C::Affine) -> Self {
        let r2 = r.square();
        let (e1, u1, w1, x1) = (
            C::Affine::ADDITIVE_IDENTITY,
            C::Scalar::one(),
            C::Affine::ADDITIVE_IDENTITY,
            r1cs.x(),
        );
        let (e2, u2, w2, x2) = (self.commit_e, self.u, self.commit_w, self.x.clone());

        let commit_e = (e1 + commit_t * r + e2 * r2).into();
        let u = u1 + r * u2;
        let commit_w = (w1 + w2 * r).into();
        let x = x1 + x2 * r;

        Self {
            commit_w,
            commit_e,
            u,
            x,
        }
    }

    pub(crate) fn absorb_by_transcript<const ROUNDS: usize>(
        &self,
        transcript: &mut MimcRO<ROUNDS, C::Base>,
    ) {
        transcript.append_point(self.commit_w);
        transcript.append_point(self.commit_e);
        transcript.append(self.u.into());
        for x in &self.x.get() {
            transcript.append(C::Base::from(*x));
        }
    }

    pub fn hash(
        &self,
        i: usize,
        z_0: &DenseVectors<C::Scalar>,
        z_i: &DenseVectors<C::Scalar>,
    ) -> C::Scalar {
        let commit_e = self.commit_e.to_extended();
        let commit_w = self.commit_w.to_extended();
        MimcRO::<MIMC_ROUNDS, C::Scalar>::default().hash_vec(
            vec![
                vec![C::Scalar::from(i as u64)],
                z_0.get(),
                z_i.get(),
                vec![self.u],
                self.x.get(),
                vec![
                    commit_e.get_x().into(),
                    commit_e.get_y().into(),
                    commit_e.get_z().into(),
                ],
                vec![
                    commit_w.get_x().into(),
                    commit_w.get_y().into(),
                    commit_w.get_z().into(),
                ],
            ]
            .concat(),
        )
    }
}
