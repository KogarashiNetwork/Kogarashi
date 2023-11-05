use crate::constraint_system::ConstraintSystem;
use crate::error::Error;

use zkstd::common::{Debug, TwistedEdwardsAffine};

/// circuit trait
pub trait Circuit<C: TwistedEdwardsAffine>: Default + Debug {
    fn synthesize(&self, constraint_system: &mut ConstraintSystem<C>) -> Result<(), Error>;
}
