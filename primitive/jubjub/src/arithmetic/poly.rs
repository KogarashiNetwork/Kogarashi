use crate::coordinate::Projective;
use crate::fr::Fr;
use crate::interface::Coordinate;
use parity_scale_codec::alloc::vec::Vec;

pub struct Polynomial(pub Vec<Fr>);

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
