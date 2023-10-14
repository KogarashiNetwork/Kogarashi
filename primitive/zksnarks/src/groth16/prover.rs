use super::constraint::Constraint;
use super::wire::Wire;
use hashbrown::HashMap;
use zkstd::common::{Field, Vec};

pub struct Prover<F: Field> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<F>>,
    pub(crate) instance: HashMap<Wire, F>,
    pub(crate) witness: HashMap<Wire, F>,
}

impl<F: Field> Prover<F> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof(&mut self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.evaluate(&self.instance, &self.witness))
    }
}
