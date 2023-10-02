use parity_scale_codec::{Decode, Encode};
use zkstd::common::{CurveAffine, CurveGroup};

/// polynomial commitment expresses as affine coordinate
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Decode, Encode)]
pub struct Commitment<A: CurveAffine>(pub A);

impl<A: CurveAffine> Commitment<A> {
    pub fn new(value: <A as CurveGroup>::Extended) -> Self {
        Self(A::from(value))
    }
}
