use zkstd::common::{TwistedEdwardsAffine, Vec};

use crate::groth16::wire::Wire;

/// constraint system trait
pub trait ConstraintSystem<C: TwistedEdwardsAffine> {
    /// init constraint system
    fn new() -> Self;

    /// return constraints length
    fn m(&self) -> usize;

    /// return public inputs and outputs
    fn instance(&self) -> Vec<C::Range>;

    /// allocate instance
    fn alloc_instance(&mut self, instance: C::Range) -> Wire;

    /// allocate witness
    fn alloc_witness(&mut self, witness: C::Range) -> Wire;
}
