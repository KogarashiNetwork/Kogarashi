use parity_scale_codec::{Decode, Encode};
use zkstd::common::{Affine, CurveGroup};

/// polynomial commitment expresses as affine coordinate
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Decode, Encode)]
pub struct Commitment<A: Affine>(pub A);

impl<A: Affine> Commitment<A> {
    pub fn new(value: <A as CurveGroup>::Extended) -> Self {
        Self(A::from(value))
    }
}
