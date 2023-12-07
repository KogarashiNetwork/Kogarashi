mod gadget;
pub mod prelude;

use crate::common::{BNAffine, Deserialize, PrimeField, Serialize};

pub trait CircuitDriver: Clone {
    const NUM_BITS: u16;
    // curve affine
    type Affine: BNAffine<Scalar = Self::Scalar, Base = Self::Base>;

    // curve base field
    type Base: PrimeField + From<Self::Scalar> + Serialize + for<'de> Deserialize<'de>;

    // curve scalar field
    type Scalar: PrimeField + From<Self::Base> + Serialize + for<'de> Deserialize<'de>;

    // bn curve 3b param
    fn b3() -> Self::Base;
}
