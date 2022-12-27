use super::{comp::ParityCmp, curve::Affine, field::PrimeField, Projective};

/// pairing field
pub trait PairingField: PrimeField + ParityCmp {}

/// pairing function range field
pub trait PairingRange: ExtentionField {
    type G1Affine: Affine;

    type G2Coeff: ParityCmp;

    fn untwist(self, coeffs: Self::G2Coeff, g1: Self::G1Affine) -> Self;

    fn conjugate(self) -> Self;

    fn final_exp(self) -> Self;
}

/// extention field
pub trait ExtentionField: ParityCmp {}

/// G2 group pairing interface
pub trait G2Pairing: Projective {
    type PairingRange: PairingRange;

    type PairingCoeff: ParityCmp;

    type PairingRepr: ParityCmp;

    fn double_eval(self) -> Self::PairingCoeff;

    fn add_eval(self, rhs: Self) -> Self::PairingCoeff;
}

/// pairing abstraction
pub trait Pairing {
    // g1 group affine point
    type G1Affine: Affine;

    // g2 group affine point
    type G2Affine: Affine;

    // g1 group projective point
    type G1Projective: Projective;

    // g2 group projective point
    type G2Projective: G2Pairing;

    type G2PairngRepr: ParityCmp;

    // range of pairing function
    type PairingRange: PairingRange;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange;

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2PairngRepr) -> Self::PairingRange;
}
