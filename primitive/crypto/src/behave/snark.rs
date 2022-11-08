// This trait resresents zkSNARKs trait

use super::{comp::ParityCmp, field::PrimeField};

#[cfg(feature = "std")]
use super::{algebra::Ring, comp::ParallelCmp};

/// This is pairing field
/// This is used for pairing
pub trait PairingField: PrimeField + ParityCmp {}

/// This is fft field
/// This is used for fft and has roots of unity
#[cfg(feature = "std")]
pub trait FftField: PrimeField + ParallelCmp {
    const ROOT_OF_UNITY: [u64; 4];
}

/// This is polynomial
/// This has fft functionality and represents polynomial ring
#[cfg(feature = "std")]
pub trait Polynomial: Ring + ParallelCmp {
    // domain of polynomial
    type Domain: FftField;

    fn evaluate(self, at: Self::Domain) -> Self::Domain;
}