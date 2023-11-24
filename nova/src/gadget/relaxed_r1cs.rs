use r1cs::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};

use crate::relaxed_r1cs::RelaxedR1csInstance;

pub(crate) struct RelaxedR1csAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C>,
    pub(crate) commit_e: PointAssignment<C>,
    pub(crate) u: FieldAssignment<C>,
    pub(crate) x: Vec<FieldAssignment<C>>,
}

impl<C: CircuitDriver> RelaxedR1csAssignment<C> {
    pub(crate) fn witness(cs: &mut R1cs<C>, relaxed_r1cs: RelaxedR1csInstance<C>) {}
}
