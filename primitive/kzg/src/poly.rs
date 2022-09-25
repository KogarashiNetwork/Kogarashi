use parity_scale_codec::alloc::vec::Vec;
use zero_jubjub::coordinate::Projective;
use zero_jubjub::fr::Fr;
use zero_jubjub::interface::Coordinate;

pub(crate) struct Polynomial(pub Vec<Fr>);

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
