use crate::driver::scalar_as_base;
use crate::hash::{MimcRO, MIMC_ROUNDS};
use crate::{PedersenCommitment, R1csShape};
use std::any::type_name;
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{BNAffine, BNProjective, CurveGroup, Group, IntGroup, PrimeField, Ring};
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
    pub(crate) fn new(x: DenseVectors<C::Scalar>) -> Self {
        Self {
            commit_w: C::Affine::ADDITIVE_IDENTITY,
            commit_e: C::Affine::ADDITIVE_IDENTITY,
            u: C::Scalar::one(),
            x,
        }
    }

    /// Initializes a new `RelaxedR1CSInstance` from an `R1CSInstance`
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
        instance: &RelaxedR1csInstance<C>,
        r: C::Scalar,
        commit_t: C::Affine,
    ) -> Self {
        let r2 = r.square();
        dbg!(type_name::<<C as CircuitDriver>::Scalar>());
        dbg!(r2);
        let (e1, u1, w1, x1) = (
            C::Affine::ADDITIVE_IDENTITY,
            C::Scalar::one(),
            C::Affine::ADDITIVE_IDENTITY,
            instance.x(),
        );
        let (e2, u2, w2, x2) = (self.commit_e, self.u, self.commit_w, self.x());

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
        let commit_e = self.commit_e.to_extended();
        let commit_w = self.commit_w.to_extended();
        MimcRO::<MIMC_ROUNDS, C>::default().hash_vec(
            vec![
                vec![E::Scalar::from(i as u64)],
                z_0.get(),
                z_i.get(),
                vec![scalar_as_base::<C>(self.u)],
                self.x.iter().map(|x| scalar_as_base::<C>(x)).collect(),
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
