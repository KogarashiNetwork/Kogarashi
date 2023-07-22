use parity_scale_codec::{Decode, Encode};
use zkstd::common::Pairing;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Decode, Encode)]
pub struct Commitment<P: Pairing>(pub P::G1Affine);

impl<P: Pairing> Commitment<P> {
    pub fn new(value: P::G1Projective) -> Self {
        Self(P::G1Affine::from(value))
    }
}
