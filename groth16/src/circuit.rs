use crate::error::Error;

use r1cs::prelude::{GrumpkinDriver, R1cs};
use zkstd::common::Debug;

/// circuit trait
pub trait Circuit: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<GrumpkinDriver>) -> Result<(), Error>;
}
