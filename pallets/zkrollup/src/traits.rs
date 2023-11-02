// < HB SBP M2 review:
//
// generally these kind of definitions are contained in a types.rs, leaving the implementations to impls.rs
//
// >
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_plonk::FullcodecRng;

pub trait Rollup {
    type F;
    type Transaction;
    type Batch;
    type PublicKey;

    fn state_root() -> Self::F;

    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;
}
