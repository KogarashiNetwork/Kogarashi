use crate::transcript::{absorb_commitment_in_ro, PoseidonRO};
use r1cs::{CircuitDriver, DenseVectors, R1cs};
use zkstd::common::{Group, PrimeField, Ring};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub(crate) fn default(x: DenseVectors<C::Scalar>) -> Self {
        Self {
            commit_w: C::Affine::ADDITIVE_IDENTITY,
            commit_e: C::Affine::ADDITIVE_IDENTITY,
            u: C::Scalar::one(),
            x,
        }
    }

    pub(crate) fn fold(&self, r1cs: &R1cs<C>, r: C::Scalar, commit_t: C::Affine) -> Self {
        let r2 = r.square();
        let (e1, u1, w1, x1) = (
            C::Affine::ADDITIVE_IDENTITY,
            C::Scalar::one(),
            C::Affine::ADDITIVE_IDENTITY,
            DenseVectors::new(r1cs.x()),
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

    pub(crate) fn absorb_in_ro(&self, ro: &mut PoseidonRO<C::Base, C::Scalar>) {
        absorb_commitment_in_ro::<C>(self.commit_w, ro);
        absorb_commitment_in_ro::<C>(self.commit_e, ro);
        ro.absorb(C::Base::from(self.u));
        for x in &self.x.get() {
            ro.absorb(C::Base::from(*x));
        }
    }
}
