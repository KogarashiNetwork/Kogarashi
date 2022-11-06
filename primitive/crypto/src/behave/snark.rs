// This trait resresents zkSNARKs trait

use super::{
    algebra::Ring,
    comp::{ParallelCmp, ParityCmp},
    field::PrimeField,
};

/// This is fft field
/// This is used for fft and has roots of unity
pub trait FftField: PrimeField + ParallelCmp {
    const ROOT_OF_UNITY: [u64; 4];
}

/// This is pairing field
/// This is used for pairing
pub trait PairingField: PrimeField + ParityCmp {}

/// This is polynomial
/// This has fft functionality and represents polynomial ring
pub trait Polynomial: Ring + ParallelCmp {
    // domain of polynomial
    type Domain: FftField;

    fn evaluate(self, at: Self::Domain) -> Self::Domain;
}
