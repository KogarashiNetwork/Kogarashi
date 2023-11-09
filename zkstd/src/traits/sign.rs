use super::{FftField, TwistedEdwardsAffine, TwistedEdwardsExtended};

use core::fmt::Debug;
use parity_scale_codec::{Decode, EncodeLike};

pub trait SigUtils<const L: usize>: Sized {
    const LENGTH: usize = L;

    fn to_bytes(self) -> [u8; L];

    fn from_bytes(bytes: [u8; L]) -> Option<Self>;
}

pub trait RedDSA: Copy + Debug + Default + Ord + PartialEq {
    type Base: FftField + SigUtils<32> + EncodeLike + Decode;

    type Scalar: FftField + SigUtils<32> + Into<Self::Base>;

    // affine point
    type Affine: TwistedEdwardsAffine<Extended = Self::Extended, Base = Self::Base, Scalar = Self::Scalar>
        + SigUtils<32>;

    // extend point
    type Extended: TwistedEdwardsExtended<Affine = Self::Affine, Base = Self::Base, Scalar = Self::Scalar>
        + Ord
        + SigUtils<32>;
}
