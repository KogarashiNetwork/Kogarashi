use crate::types::*;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use zkstd::common::{Pairing, Vec};

/// Abstraction over a plonk zk-SNARKs system
pub trait Plonk<P: Pairing> {
    /// The plonk circuit customized by developer
    type CustomCircuit: Circuit<P::JubjubAffine>;

    /// The public parameters generation function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn verify(proof: Proof<P>, public_inputs: Vec<P::ScalarField>) -> DispatchResultWithPostInfo;
}
