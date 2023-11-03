use crate::{
    circuit::Circuit, constraint_system::ConstraintSystem, error::Error,
    public_params::PublicParameters,
};

use zkstd::common::{Pairing, TwistedEdwardsAffine};

/// key pair trait
/// zkp prover and verifier key
pub trait Keypair<P: Pairing, A: TwistedEdwardsAffine, C: Circuit<A>> {
    type PublicParameters: PublicParameters<P>;
    type Prover;
    type Verifier;
    type ConstraintSystem: ConstraintSystem<A>;

    fn compile(pp: &Self::PublicParameters) -> Result<(Self::Prover, Self::Verifier), Error>;
}
