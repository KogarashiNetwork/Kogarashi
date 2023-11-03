use super::{FftField, TwistedEdwardsAffine, TwistedEdwardsExtended};

use core::fmt::Debug;
use parity_scale_codec::{Decode, EncodeLike};

pub trait SigUtils<const L: usize>: Sized {
    const LENGTH: usize = L;

    fn to_bytes(self) -> [u8; L];

    fn from_bytes(bytes: [u8; L]) -> Option<Self>;
}

pub trait RedDSA: Copy + Debug + Default + Ord + PartialEq {
    type Range: FftField + Eq + PartialEq + SigUtils<32> + EncodeLike + Decode;

    type Scalar: FftField + Eq + PartialEq + SigUtils<32> + Into<Self::Range>;

    // affine point
    type Affine: TwistedEdwardsAffine<Extended = Self::Extended, Range = Self::Range, Scalar = Self::Scalar>
        + PartialEq
        + Eq
        + SigUtils<32>;

    // extend point
    type Extended: TwistedEdwardsExtended<Affine = Self::Affine, Range = Self::Range, Scalar = Self::Scalar>
        + PartialEq
        + Eq
        + Ord
        + SigUtils<32>;
}
