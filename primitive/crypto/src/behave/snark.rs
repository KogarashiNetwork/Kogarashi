// This trait resresents zkSNARKs trait

use super::{algebra::Field, basic::Basic};

/// This is fft field
/// This is used for fft and has roots of unity
pub trait FftField: Field + Basic {
    const ROOT_OF_UNITY: [u64; 4];
}

/// This is pairing field
/// This is used for pairing
pub trait PairingField: Field + Basic {}
