use crate::error::Error;

use grumpkin::driver::GrumpkinDriver;
use zkstd::circuit::prelude::R1cs;
use zkstd::common::Debug;

/// circuit trait
pub trait Circuit: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<GrumpkinDriver>) -> Result<(), Error>;
}
