use crate::driver::scalar_as_base;
use crate::hash::{MimcRO, MIMC_ROUNDS};
use crate::{PedersenCommitment, R1csShape};
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{CurveGroup, Group, IntGroup, Ring};
use zkstd::matrix::DenseVectors;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R1csInstance<C: CircuitDriver> {
    /// commitment for witness vectors
    pub(crate) commit_w: C::Affine,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> R1csInstance<C> {
    pub fn new(shape: &R1csShape<C>, commit_w: C::Affine, x: Vec<C::Scalar>) -> Self {
        assert_eq!(shape.l(), x.len());
        Self {
            commit_w,
            x: DenseVectors::new(x),
        }
    }

    pub(crate) fn dummy(x_len: usize) -> Self {
        Self {
            commit_w: C::Affine::ADDITIVE_IDENTITY,
            x: DenseVectors::zero(x_len),
        }
    }

    pub(crate) fn x(&self) -> DenseVectors<C::Scalar> {
        self.x.clone()
    }
}

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
    /// Initializes a new `RelaxedR1csInstance` from an `R1csInstance`
    pub fn from_r1cs_instance(
        ck: &PedersenCommitment<C::Affine>,
        shape: &R1csShape<C>,
        instance: &R1csInstance<C>,
    ) -> Self {
        let mut r_instance = RelaxedR1csInstance::dummy(shape.l());
        r_instance.commit_w = instance.commit_w;
        r_instance.u = C::Scalar::one();
        r_instance.x = instance.x.clone();
        r_instance
    }

    pub(crate) fn dummy(x_len: usize) -> Self {
        Self {
            commit_w: C::Affine::ADDITIVE_IDENTITY,
            commit_e: C::Affine::ADDITIVE_IDENTITY,
            u: C::Scalar::zero(),
            x: DenseVectors::zero(x_len),
        }
    }

    pub(crate) fn u(&self) -> C::Scalar {
        self.u
    }

    pub(crate) fn x(&self) -> DenseVectors<C::Scalar> {
        self.x.clone()
    }

    pub(crate) fn fold(
        &self,
        instance: &R1csInstance<C>,
        r: C::Scalar,
        commit_t: C::Affine,
    ) -> Self {
        let (e1, u1, w1, x1) = (self.commit_e, self.u, self.commit_w, self.x());
        let (w2, x2) = (instance.commit_w, instance.x());

        let commit_e = (e1 + commit_t * r).into();
        let u = u1 + r;
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
        transcript: &mut MimcRO<ROUNDS, C>,
    ) {
        transcript.append_point(self.commit_w);
        transcript.append_point(self.commit_e);
        transcript.append(scalar_as_base::<C>(self.u));
        for x in &self.x.get() {
            transcript.append(scalar_as_base::<C>(*x));
        }
    }

    pub fn hash<E: CircuitDriver<Base = C::Scalar, Scalar = C::Base>>(
        &self,
        i: usize,
        z_0: &DenseVectors<E::Scalar>,
        z_i: &DenseVectors<E::Scalar>,
    ) -> C::Scalar {
        MimcRO::<MIMC_ROUNDS, C>::default().hash_vec(
            vec![
                vec![E::Scalar::from(i as u64)],
                z_0.get(),
                z_i.get(),
                vec![scalar_as_base::<C>(self.u)],
                self.x.iter().map(|x| scalar_as_base::<C>(x)).collect(),
                vec![
                    self.commit_e.get_x(),
                    self.commit_e.get_y(),
                    if self.commit_e.is_identity() {
                        C::Base::zero()
                    } else {
                        C::Base::one()
                    },
                ],
                vec![
                    self.commit_w.get_x(),
                    self.commit_w.get_y(),
                    if self.commit_w.is_identity() {
                        C::Base::zero()
                    } else {
                        C::Base::one()
                    },
                ],
            ]
            .concat(),
        )
    }
}
