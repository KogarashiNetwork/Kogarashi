use super::constraint::Constraint;
use super::wire::Wire;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::Groth16;
use hashbrown::HashMap;
use zkstd::common::{CurveGroup, Pairing, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<<P::JubjubAffine as CurveGroup>::Range>>,
    pub(crate) instance: HashMap<Wire, <P::JubjubAffine as CurveGroup>::Range>,
    pub(crate) witness: HashMap<Wire, <P::JubjubAffine as CurveGroup>::Range>,
}

impl<P: Pairing> Prover<P> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C>(&mut self, circuit: C) -> Result<bool, Error>
    where
        C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>,
    {
        let mut prover = Groth16::<P::JubjubAffine>::initialize();
        circuit.synthesize(&mut prover)?;

        Ok(prover
            .constraints
            .iter()
            .all(|constraint| constraint.evaluate(&prover.instance, &prover.witness)))
    }
}
