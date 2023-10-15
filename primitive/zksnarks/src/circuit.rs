use zkstd::common::{Debug, TwistedEdwardsAffine};

use crate::constraint_system::ConstraintSystem;
use crate::error::Error;

/// circuit trait
pub trait Circuit<C: TwistedEdwardsAffine>: Default + Debug {
    type ConstraintSystem: ConstraintSystem<C>;

    fn synthesize(&self, constraint_system: &mut Self::ConstraintSystem) -> Result<(), Error>;
}
