mod gadget;
pub mod prelude;

use crate::common::{BNAffine, Deserialize, PrimeField, Serialize};
use core::fmt::Debug;
use parity_scale_codec::{Decode, Encode};

pub trait CircuitDriver: Clone + Debug + Default + Decode + Encode + PartialEq + Eq {
    const ORDER_STR: &'static str;
    const NUM_BITS: u16;
    // curve affine
    type Affine: BNAffine<Scalar = Self::Scalar, Base = Self::Base>;

    // curve base field
    type Base: PrimeField + From<Self::Scalar> + Serialize + for<'de> Deserialize<'de>;

    // curve scalar field
    type Scalar: PrimeField + From<Self::Base> + Serialize + for<'de> Deserialize<'de>;
    // bn curve 3b param
    fn b3() -> Self::Scalar;
}
