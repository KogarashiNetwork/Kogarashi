use crate::types::*;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use zkstd::common::{Pairing, TwistedEdwardsAffine, Vec};

/// Abstraction over a plonk zk-SNARKs system
pub trait Plonk<P: Pairing, A: TwistedEdwardsAffine<Range = P::ScalarField>> {
    /// The plonk circuit customized by developer
    type CustomCircuit: Circuit<A>;

    /// The public parameters generation function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn verify(proof: Proof<P>, public_inputs: Vec<A::Range>) -> DispatchResultWithPostInfo;
}
