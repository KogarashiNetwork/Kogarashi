use parity_scale_codec::{Decode, Encode};
use zkstd::common::WeierstrassAffine;

/// polynomial commitment expresses as affine coordinate
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Decode, Encode)]
pub struct Commitment<A: WeierstrassAffine>(pub A);

impl<A: WeierstrassAffine> Commitment<A> {
    pub fn new(value: A::Extended) -> Self {
        Self(A::from(value))
    }
}
