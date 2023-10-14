use crate::{circuit::Circuit, error::Error};

use zkstd::common::{RngCore, TwistedEdwardsAffine, Vec};

/// prover trait
pub trait Prover<C: TwistedEdwardsAffine> {
    type Proof;

    type Circuit: Circuit<C>;

    type Rng: RngCore;

    /// create proof for circuit instance and return proof and public inputs/outputs
    fn create_proof(
        &self,
        rng: &mut Self::Rng,
        circuit: &Self::Circuit,
    ) -> Result<(Self::Proof, Vec<C::Range>), Error>;
}
