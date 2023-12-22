use crate::types::*;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;

/// Abstraction over an ivc system
pub trait IvcVerifier<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    /// The proof verify function
    /// This is the dispatchable function and assumed to be called by other pallet as API
    fn verify(
        proof: RecursiveProof<E1, E2, FC1, FC2>,
        pp: PublicParams<E1, E2, FC1, FC2>,
    ) -> DispatchResultWithPostInfo;
}
