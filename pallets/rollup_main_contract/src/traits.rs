use frame_support::pallet_prelude::DispatchResultWithPostInfo;

/// Confidential transfer by coupling encrypted currency and plonk
pub trait MainContract {
    type F;
    type Transaction;
    type Batch;
    type Proof;
    type PublicKey;

    fn state_root() -> Self::F;
}
