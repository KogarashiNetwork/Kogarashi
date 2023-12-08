use crate::error::Error;
use bn_254::driver::Bn254Driver;

use zkstd::circuit::prelude::R1cs;
use zkstd::common::Debug;

/// circuit trait
pub trait Circuit: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<Bn254Driver>) -> Result<(), Error>;
}
