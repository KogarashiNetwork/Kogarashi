use parity_scale_codec::alloc::vec::Vec;
use zero_crypto::behave::*;

pub(crate) struct Polynomial(pub Vec<FftField>);

impl Polynomial {
    pub(crate) fn evaluate(self, at: Fr) -> Projective {
        self.0
            .iter()
            .rev()
            .fold(Projective::identity(), |acc, coeff| {
                acc * at + *coeff * Projective::identity()
            })
    }
}
