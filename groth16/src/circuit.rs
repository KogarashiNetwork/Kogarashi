use crate::error::Error;

use r1cs::R1cs;
use zkstd::common::{Debug, TwistedEdwardsAffine};

/// circuit trait
pub trait Circuit<C: TwistedEdwardsAffine>: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<C::Range>) -> Result<(), Error>;
}
