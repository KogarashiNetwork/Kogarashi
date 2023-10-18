use parity_scale_codec::{Decode, EncodeLike};

use super::{FftField, TwistedEdwardsAffine, TwistedEdwardsExtended};

pub trait SigUtils<const L: usize>: Sized {
    const LENGTH: usize = L;

    fn to_bytes(self) -> [u8; L];

    fn from_bytes(bytes: [u8; L]) -> Option<Self>;
}

pub trait RedDSA: Copy {
    type ScalarField: FftField + Eq + PartialEq + EncodeLike + Decode + SigUtils<32>;

    type JubjubScalar: FftField + Eq + PartialEq + SigUtils<32> + Into<Self::ScalarField>;

    // Jubjub affine point
    type JubjubAffine: TwistedEdwardsAffine<
            Extended = Self::JubjubExtended,
            Range = Self::ScalarField,
            Scalar = Self::JubjubScalar,
        > + PartialEq
        + Eq
        + SigUtils<32>;

    // Jubjub extend point
    type JubjubExtended: TwistedEdwardsExtended<
            Affine = Self::JubjubAffine,
            Range = Self::ScalarField,
            Scalar = Self::JubjubScalar,
        > + PartialEq
        + Eq
        + Ord
        + SigUtils<32>;
}
