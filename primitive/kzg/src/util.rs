use zkstd::behave::Ring;
use zkstd::common::Pairing;
use zkstd::common::Vec;

pub(crate) fn powers_of<P: Pairing>(
    scalar: &P::ScalarField,
    max_degree: usize,
) -> Vec<P::ScalarField> {
    let mut powers = Vec::with_capacity(max_degree + 1);
    powers.push(P::ScalarField::one());
    for i in 1..=max_degree {
        powers.push(powers[i - 1] * scalar);
    }
    powers
}
