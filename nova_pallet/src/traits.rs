use crate::FullcodecRng;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use zknova::{FunctionCircuit, RecursiveProof};
use zkstd::circuit::CircuitDriver;

/// Abstraction over an ivc system
pub trait Ivc<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    /// The public parameters generation function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn trusted_setup(rng: FullcodecRng) -> DispatchResultWithPostInfo;

    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn verify(proof: RecursiveProof<E1, E2, FC1, FC2>) -> DispatchResultWithPostInfo;
}
