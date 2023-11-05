use core::{fmt::Debug, iter::Sum, ops::Add};

use parity_scale_codec::{Decode, Encode, EncodeLike};

use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
    curve::CurveAffine,
    sign::SigUtils,
    FftField, Group, WeierstrassAffine, WeierstrassProjective,
};

/// extension field
pub trait ExtensionField: Field + Basic + ParityCmp {
    fn mul_by_nonresidue(self) -> Self;
}

/// pairing function range field
pub trait PairingRange: ExtensionField {
    type G1Affine: CurveAffine;
    type G2Coeff: ParityCmp;
    type QuadraticField: ExtensionField;
    type Gt: Group + Debug;

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
pub trait G2Pairing: WeierstrassProjective {
    type PairingRange: PairingRange;
    type PairingCoeff: ParityCmp;
    type PairingRepr: ParityCmp;
    type G2Affine: CurveAffine;

    fn double_eval(&mut self) -> Self::PairingCoeff;

    fn add_eval(&mut self, rhs: Self::G2Affine) -> Self::PairingCoeff;
}

/// pairing abstraction
pub trait Pairing:
    Send + Sync + Clone + Copy + Debug + Eq + PartialEq + Ord + Default + Encode + Decode
{
    // g1 group affine point
    type G1Affine: WeierstrassAffine<
            Affine = Self::G1Affine,
            Extended = Self::G1Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G1Projective>
        + Add<Self::G1Projective, Output = Self::G1Projective>
        + SigUtils<48>
        + PartialEq
        + Eq
        + Sync
        + Send
        + Encode
        + Decode;
    // g2 group affine point
    type G2Affine: WeierstrassAffine<
            Affine = Self::G2Affine,
            Extended = Self::G2Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G2Projective>
        + PartialEq
        + Eq
        + Encode
        + Decode;
    // g1 group projective point
    type G1Projective: WeierstrassProjective<
            Affine = Self::G1Affine,
            Extended = Self::G1Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G1Affine>
        + Copy
        + Sum
        + Send
        + Sync
        + PartialEq
        + Eq;
    // g2 group projective point
    type G2Projective: WeierstrassProjective<
            Affine = Self::G2Affine,
            Extended = Self::G2Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G2Affine>
        + G2Pairing
        + PartialEq
        + Eq;

    // g2 pairing representation
    type G2PairngRepr: From<Self::G2Affine> + ParityCmp + Debug + Eq + PartialEq + Clone + Default;
    // range of pairing function
    type PairingRange: PairingRange + Debug + Eq + PartialEq;
    type Gt: Group + Debug + Eq + PartialEq;
    // Used for commitment
    type ScalarField: FftField + Eq + PartialEq + EncodeLike + Decode + SigUtils<32> + Sum;

    const PARAM_D: Self::ScalarField;
    const X: u64;
    const X_IS_NEGATIVE: bool;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::Gt;

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;

    fn multi_miller_loop(pairs: &[(Self::G1Affine, Self::G2PairngRepr)]) -> Self::PairingRange;
}
