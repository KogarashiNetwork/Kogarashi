use crate::error::Error;

use bls_12_381::Fr;
use r1cs::R1cs;
use zkstd::common::Debug;

/// circuit trait
pub trait Circuit: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<Fr>) -> Result<(), Error>;
}
