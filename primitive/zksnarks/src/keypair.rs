use crate::{circuit::Circuit, public_params::PublicParameters};

use zkstd::common::Pairing;

/// key pair trait
/// zkp proving and verification key
pub trait Keypair<P: Pairing> {
    type Circuit: Circuit<P::JubjubAffine>;
    type PublicParameters: PublicParameters<P>;
    type ProvingKey;
    type VerificationKey;

    fn new(
        pp: Self::PublicParameters,
        circuit: Self::Circuit,
    ) -> (Self::ProvingKey, Self::VerificationKey);
}
