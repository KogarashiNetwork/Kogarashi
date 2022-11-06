use super::{algebra::Field, basic::Basic};

pub trait FftField: Field + Basic {
    const ROOT_OF_UNITY: [u64; 4];
}

pub trait PairingField: Field + Basic {}
