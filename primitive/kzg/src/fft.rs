use crate::poly::Polynomial;
use rayon::{join, prelude::*};
use zero_crypto::common::*;

#[derive(Clone, Debug)]
pub struct Fft<F: FftField> {
    // polynomial degree 2^k
    k: u32,
    // generator of order 2^{k - 1} multiplicative group used as twiddle factors
    twiddle_factors: Vec<F>,
    // multiplicative group generator inverse
    inv_twiddle_factors: Vec<F>,
    // n inverse for inverse discrete fourier transform
    inv: F,
}

impl<F: FftField> Fft<F> {
    pub fn new(k: u32) -> Self {
        let n = 1 << k;
        let half_n = n / 2;
        let mut g = F::ROOT_OF_UNITY;
        let inv = F::from_u64(n).invert().unwrap();
        let s = F::S;

        // adjust cofactor
        for _ in 0..s - k as usize {
            g = g.square()
        }

        let g_inv = g.invert().unwrap();

        // compute twiddle factors
        let twiddle_factors = (0..half_n as usize)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g;
                Some(tw)
            })
            .collect::<Vec<_>>();

        // compute inverse twiddle factors
        let inv_twiddle_factors = (0..half_n as usize)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g_inv;
                Some(tw)
            })
            .collect::<Vec<_>>();

        Fft {
            k,
            twiddle_factors,
            inv_twiddle_factors,
            inv,
        }
    }

    // perform classic discrete fourier transform
    pub fn dft(&self, coeffs: &mut Polynomial<F>) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        swap_bit_reverse(coeffs, n, self.k);

        classic_fft_arithmetic(coeffs, n, 1, &self.twiddle_factors)
    }

    // perform classic inverse discrete fourier transform
    pub fn idft(&self, coeffs: &mut Polynomial<F>) {
        let n = 1 << self.k;
        assert_eq!(coeffs.len(), n);

        swap_bit_reverse(coeffs, n, self.k);

        classic_fft_arithmetic(coeffs, n, 1, &self.inv_twiddle_factors);
        coeffs.par_iter_mut().for_each(|coeff| *coeff *= self.inv)
    }
}

// classic fft using divide and conquer algorithm
fn classic_fft_arithmetic<F: FftField>(
    coeffs: &mut Polynomial<F>,
    n: usize,
    twiddle_chunk: usize,
    twiddles: &[F],
) {
    if n == 2 {
        let t = coeffs[1];
        coeffs[1] = coeffs[0];
        coeffs[0] += t;
        coeffs[1] -= t;
    } else {
        let (left, right) = coeffs.split_at_mut(n / 2);
        join(
            || classic_fft_arithmetic(left, n / 2, twiddle_chunk * 2, twiddles),
            || classic_fft_arithmetic(right, n / 2, twiddle_chunk * 2, twiddles),
        );
        butterfly_arithmetic(left, right, twiddle_chunk, twiddles)
    }
}

pub(crate) fn swap_bit_reverse<F: FftField>(a: &mut [F], n: usize, k: u32) {
    assert!(k <= 64);
    let diff = 64 - k;
    for i in 0..n as u64 {
        let ri = i.reverse_bits() >> diff;
        if i < ri {
            a.swap(ri as usize, i as usize);
        }
    }
}

pub(crate) fn butterfly_arithmetic<F: FftField>(
    left: &mut [F],
    right: &mut [F],
    twiddle_chunk: usize,
    twiddles: &[F],
) {
    // case when twiddle factor is one
    let t = right[0];
    right[0] = left[0];
    left[0] += t;
    right[0] -= t;

    left.iter_mut()
        .zip(right.iter_mut())
        .enumerate()
        .skip(1)
        .for_each(|(i, (a, b))| {
            let mut t = *b;
            t *= twiddles[i * twiddle_chunk];
            *b = *a;
            *a += t;
            *b -= t;
        });
}

#[cfg(test)]
mod tests {
    use super::Fft;
    use proptest::prelude::*;
    use proptest::std_facade::vec;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::behave::PrimeField;
    use zero_crypto::common::Vec;
    use zero_jubjub::fr::Fr;

    prop_compose! {
        fn arb_poly(k: u32)(bytes in vec![[any::<u8>(); 16]; 1 << k as usize]) -> Vec<Fr> {
            (0..(1 << k)).map(|i| Fr::random(XorShiftRng::from_seed(bytes[i]))).collect::<Vec<Fr>>()
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn classic_fft_test(mut poly_a in arb_poly(4)) {
            let poly_b = poly_a.clone();
            let classic_fft = Fft::new(4);

            classic_fft.dft(&mut poly_a);
            classic_fft.idft(&mut poly_a);

            assert_eq!(poly_a, poly_b)
        }
    }
}
