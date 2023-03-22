use zero_crypto::common::Pairing;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Commitment<P: Pairing>(pub P::G1Affine);

impl<P: Pairing> Commitment<P> {
    pub fn new(value: P::G1Projective) -> Self {
        Self(P::G1Affine::from(value))
    }
}
