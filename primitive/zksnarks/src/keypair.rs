use crate::{
    circuit::Circuit, constraint_system::ConstraintSystem, error::Error,
    public_params::PublicParameters,
};

use zkstd::common::Pairing;

/// key pair trait
/// zkp prover and verifier key
pub trait Keypair<P: Pairing, C: Circuit<P::JubjubAffine>> {
    type PublicParameters: PublicParameters<P>;
    type Prover;
    type Verifier;
    type ConstraintSystem: ConstraintSystem<P::JubjubAffine>;

    fn compile(pp: &Self::PublicParameters) -> Result<(Self::Prover, Self::Verifier), Error>;
}
