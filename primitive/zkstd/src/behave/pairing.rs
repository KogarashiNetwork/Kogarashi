use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::Add,
};

use parity_scale_codec::{Decode, Encode};

use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
    curve::Affine,
    sign::SigUtils,
    Curve, CurveExtended, FftField, Group, Projective, TwistedEdwardsAffine, TwistedEdwardsCurve,
    TwistedEdwardsExtended, WeierstrassAffine,
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
pub trait G2Pairing: Projective {
    type PairingRange: PairingRange;
    type PairingCoeff: ParityCmp;
    type PairingRepr: ParityCmp;
    type G2Affine: Affine;

    fn double_eval(&mut self) -> Self::PairingCoeff;

    fn add_eval(&mut self, rhs: Self::G2Affine) -> Self::PairingCoeff;
}

/// pairing abstraction
pub trait Pairing:
    Send + Sync + Clone + Debug + Eq + PartialEq + Default + Encode + Decode
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
    type G1Projective: Projective<
            Affine = Self::G1Affine,
            Extended = Self::G1Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G1Affine>
        + Sum
        + Send
        + Sync
        + PartialEq
        + Eq;
    // g2 group projective point
    type G2Projective: Projective<
            Affine = Self::G2Affine,
            Extended = Self::G2Projective,
            Scalar = Self::ScalarField,
        > + From<Self::G2Affine>
        + G2Pairing
        + PartialEq
        + Eq;
    // Jubjub affine point
    type JubjubAffine: TwistedEdwardsAffine<
            Affine = Self::JubjubAffine,
            Extended = Self::JubjubExtended,
            Scalar = Self::ScalarField,
        > + PartialEq
        + Eq;

    // Jubjub extend point
    type JubjubExtended: CurveExtended<
            Affine = Self::JubjubAffine,
            Extended = Self::JubjubExtended,
            Scalar = Self::ScalarField,
        > + TwistedEdwardsExtended
        + TwistedEdwardsCurve
        + PartialEq
        + Eq;

    // g2 pairing representation
    type G2PairngRepr: From<Self::G2Affine> + ParityCmp + Debug + Eq + PartialEq + Clone;
    // range of pairing function
    type PairingRange: PairingRange + Debug + Eq + PartialEq;
    type Gt: Group + Debug + Eq + PartialEq;
    // Used for commitment
    type ScalarField: FftField
        + Sum
        + Product
        + From<<Self::JubjubExtended as Curve>::Range>
        + From<<Self::JubjubAffine as Curve>::Range>
        + Into<<Self::JubjubExtended as Curve>::Range>
        + Into<<Self::JubjubAffine as Curve>::Range>
        + Encode
        + Decode
        + Eq
        + PartialEq
        + SigUtils<32>;
    type JubjubScalar: FftField + Into<Self::ScalarField> + Eq + PartialEq + SigUtils<32>;

    const X: u64;
    const X_IS_NEGATIVE: bool;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::Gt;

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;

    fn multi_miller_loop(pairs: &[(Self::G1Affine, Self::G2PairngRepr)]) -> Self::PairingRange;
}
