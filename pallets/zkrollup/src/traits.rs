use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use pallet_plonk::FullcodecRng;

pub trait Rollup {
    type F;
    type Transaction;
    type Batch;
    type Proof;
    type PublicKey;

    fn state_root() -> Self::F;

    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo;
}
