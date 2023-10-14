use crate::groth16::constraint_system::ConstraintSystem;
use zkstd::common::{Debug, TwistedEdwardsAffine, Vec};

use crate::error::Error;

/// circuit trait
pub trait Circuit<C: TwistedEdwardsAffine>: Default + Debug {
    fn synthesize(&self, composer: &mut ConstraintSystem<C>) -> Result<(), Error>;
}

/// constraint system trait
pub trait AltConstraintSystem<C: TwistedEdwardsAffine> {
    /// return constraints length
    fn m(self) -> usize;

    /// return public inputs and outputs
    fn instance() -> Vec<C::Scalar>;
}
