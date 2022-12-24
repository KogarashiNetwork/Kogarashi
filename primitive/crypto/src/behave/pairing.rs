use super::{comp::ParityCmp, curve::Affine, field::PrimeField};

/// pairing field
pub trait PairingField: PrimeField + ParityCmp {}

/// pairing function range field
pub trait PairingRange: PairingField {
    fn final_exp(self) -> Self;
}

/// extention field
pub trait ExtentionField: ParityCmp {}

/// pairing abstraction
pub trait Pairing {
    // g1 group affine point
    type G1Affine: Affine;

    // g2 group affine point
    type G2Affine: Affine;

    // range of pairing function
    type PairngRange: ExtentionField;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairngRange;

    fn miller_loop();
}
