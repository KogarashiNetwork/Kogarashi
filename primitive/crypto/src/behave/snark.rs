// This trait resresents zkSNARKs trait

use core::ops::Mul;

use super::{
    comp::ParityCmp,
    curve::{Affine, Projective},
    field::PrimeField,
};

#[cfg(feature = "std")]
use super::{algebra::Field, comp::ParallelCmp};

/// This is pairing field
/// This is used for pairing
pub trait PairingField: PrimeField + ParityCmp {}

/// This is extention field
/// This is extention of pairing field
pub trait ExtentionField: ParityCmp {}

/// This is fft field
/// This is used for fft and has roots of unity
#[cfg(feature = "std")]
pub trait FftField: PrimeField + ParallelCmp + From<u64> {
    // factor power of two
    const S: usize;

    // 2^s th root of unity
    const ROOT_OF_UNITY: Self;

    fn zero() -> Self;

    fn one() -> Self;
}

/// This is polynomial
/// This has fft functionality and represents polynomial ring
#[cfg(feature = "std")]
pub trait Polynomial: Field + ParallelCmp {
    // domain of polynomial
    type Domain: FftField;

    fn evaluate(self, at: Self::Domain) -> Self::Domain;
}

/// This is commitment
pub trait Commitment {
    // g1 group affine point
    type G1Affine: Affine + From<Self::G1Projective>;

    // g1 group projective point
    type G1Projective: Projective
        + From<Self::G1Affine>
        + Mul<Self::ScalarField, Output = Self::G1Projective>;

    // g2 group affine point
    type G2Affine: Affine;

    // g2 group projective point
    type G2Projective: Projective;

    // scalar field of point
    type ScalarField: PrimeField;
}
