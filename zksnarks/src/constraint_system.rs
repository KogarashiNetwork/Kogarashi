use zkstd::common::TwistedEdwardsAffine;

/// constraint system trait
pub trait ConstraintSystem<C: TwistedEdwardsAffine> {
    type Wire;
    type Constraints;

    /// init constraint system
    fn initialize() -> Self;

    /// return constraints length
    fn m(&self) -> usize;

    /// allocate instance
    fn alloc_instance(&mut self, instance: C::Range) -> Self::Wire;

    /// allocate witness
    fn alloc_witness(&mut self, witness: C::Range) -> Self::Wire;
}
