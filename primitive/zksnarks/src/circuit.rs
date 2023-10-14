use zkstd::common::{Debug, Field};

use crate::error::Error;
use crate::groth16::constraint_system::ConstraintSystem;

/// circuit implementation
pub trait Circuit<F: Field>: Default + Debug {
    fn synthesize(&self, composer: &mut ConstraintSystem<F>) -> Result<(), Error>;
}
