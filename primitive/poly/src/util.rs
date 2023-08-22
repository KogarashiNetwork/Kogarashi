use zkstd::common::{PrimeField, Vec};

/// power operation for prime field
pub fn powers_of<F: PrimeField>(scalar: &F, max_degree: usize) -> Vec<F> {
    let mut powers = Vec::with_capacity(max_degree + 1);
    powers.push(F::one());
    for i in 1..=max_degree {
        powers.push(powers[i - 1] * scalar);
    }
    powers
}

/// batch inversion operation for prime field vectors
pub fn batch_inversion<F: PrimeField>(v: &mut [F]) {
    // Montgomery’s Trick and Fast Implementation of Masked AES
    // Genelle, Prouff and Quisquater
    // Section 3.2

    // First pass: compute [a, ab, abc, ...]
    let mut prod = Vec::with_capacity(v.len());
    let mut tmp = F::one();
    for f in v.iter().filter(|f| f != &&F::zero()) {
        tmp.mul_assign(f);
        prod.push(tmp);
    }

    // Invert `tmp`.
    tmp = tmp.invert().unwrap(); // Guaranteed to be nonzero.

    // Second pass: iterate backwards to compute inverses
    for (f, s) in v
        .iter_mut()
        // Backwards
        .rev()
        // Ignore normalized elements
        .filter(|f| f != &&F::zero())
        // Backwards, skip last element, fill in one for last term.
        .zip(prod.into_iter().rev().skip(1).chain(Some(F::one())))
    {
        // tmp := tmp * f; f := tmp * s = 1/f
        let new_tmp = tmp * *f;
        *f = tmp * s;
        tmp = new_tmp;
    }
}

#[cfg(test)]
mod test {
    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::Group;

    use super::*;
    #[test]
    fn test_batch_inversion() {
        let one = BlsScalar::from(1);
        let two = BlsScalar::from(2);
        let three = BlsScalar::from(3);
        let four = BlsScalar::from(4);
        let five = BlsScalar::from(5);

        let original_scalars = vec![one, two, three, four, five];
        let mut inverted_scalars = vec![one, two, three, four, five];

        batch_inversion(&mut inverted_scalars);
        for (x, x_inv) in original_scalars.iter().zip(inverted_scalars.iter()) {
            assert_eq!(x.invert().unwrap(), *x_inv);
        }
    }
}
