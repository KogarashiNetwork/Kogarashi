use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, Mul, Neg},
};

use dusk_bytes::Serializable;

use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
    curve::Affine,
    FftField, Group, Projective,
};

/// extension field
pub trait ExtensionField: Field + Basic + ParityCmp {
    fn mul_by_nonresidue(self) -> Self;
}

/// pairing function range field
pub trait PairingRange: ExtensionField {
    type G1Affine: Affine;
    type G2Coeff: ParityCmp;
    type QuadraticField: ExtensionField;
    type Gt: Group;

    fn mul_by_014(
        self,
        c0: Self::QuadraticField,
        c1: Self::QuadraticField,
        c4: Self::QuadraticField,
    ) -> Self;

    fn untwist(self, coeffs: Self::G2Coeff, g1: Self::G1Affine) -> Self;

    fn final_exp(self) -> Self::Gt;
}

/// G2 group pairing interface
pub trait G2Pairing: Projective {
    type PairingRange: PairingRange;
    type PairingCoeff: ParityCmp;
    type PairingRepr: ParityCmp;
    type G2Affine: Affine;

    fn double_eval(&mut self) -> Self::PairingCoeff;

    fn add_eval(&mut self, rhs: Self::G2Affine) -> Self::PairingCoeff;
}

/// pairing abstraction
pub trait Pairing: Send + Sync + Clone + Debug + PartialEq {
    // g1 group affine point
    type G1Affine: Affine
        + From<Self::G1Projective>
        + Mul<Self::ScalarField, Output = Self::G1Projective>
        + Add<Self::G1Projective, Output = Self::G1Projective>
        + Serializable<48>
        + PartialEq
        + Eq
        + Sync
        + Send;
    // g2 group affine point
    type G2Affine: Affine + From<Self::G2Projective> + Neg<Output = Self::G2Affine> + PartialEq + Eq;
    // g1 group projective point
    type G1Projective: Projective
        + From<Self::G1Affine>
        + Mul<Self::ScalarField, Output = Self::G1Projective>
        + Add<Self::G1Affine, Output = Self::G1Projective>
        + Sum
        + Send
        + Sync;
    // g2 group projective point
    type G2Projective: Projective
        + From<Self::G2Affine>
        + Mul<Self::ScalarField, Output = Self::G2Projective>
        + G2Pairing;
    // g2 pairing representation
    type G2PairngRepr: From<Self::G2Affine> + ParityCmp + Debug + PartialEq;
    // range of pairing function
    type PairingRange: PairingRange;
    type Gt: Group;
    // Used for commitment
    type ScalarField: FftField + Serializable<32> + Sum + Product;

    const X: u64;
    const X_IS_NEGATIVE: bool;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::Gt;

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;

    fn multi_miller_loop(pairs: &[(Self::G1Affine, Self::G2PairngRepr)]) -> Self::PairingRange;
}
