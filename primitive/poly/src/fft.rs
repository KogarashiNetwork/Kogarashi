use crate::poly::Polynomial;
use rayon::join;
use zkstd::common::{FftField, Vec};

// fft structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fft<F: FftField> {
    // polynomial degree 2^k
    n: usize,
    // generator of order 2^{k - 1} multiplicative group used as twiddle factors
    twiddle_factors: Vec<F>,
    // multiplicative group generator inverse
    inv_twiddle_factors: Vec<F>,
    // coset domain
    cosets: Vec<F>,
    // inverse coset domain
    inv_cosets: Vec<F>,
    // n inverse for inverse discrete fourier transform
    n_inv: F,
    // bit reverse index
    bit_reverse: Vec<(usize, usize)>,
    pub elements: Vec<F>,
}

// SBP-M1 review: use safe math operations

impl<F: FftField> Fft<F> {
    pub fn new(k: usize) -> Self {
        let n = 1 << k;
        let half_n = n / 2;
        let offset = 64 - k;

        // precompute twiddle factors
        let g = (0..F::S - k).fold(F::ROOT_OF_UNITY, |acc, _| acc.square());
        let twiddle_factors = (0..half_n)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g;
                Some(tw)
            })
            .collect::<Vec<_>>();

        // precompute inverse twiddle factors
        let g_inv = g.invert().unwrap();
        let inv_twiddle_factors = (0..half_n)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g_inv;
                Some(tw)
            })
            .collect::<Vec<_>>();

        // precompute cosets
        let mul_g = F::MULTIPLICATIVE_GENERATOR;
        let cosets = (0..half_n)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= mul_g;
                Some(tw)
            })
            .collect::<Vec<_>>();

        // precompute inverse cosets
        let mul_g_inv = mul_g.invert().unwrap();
        let inv_cosets = (0..n)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= mul_g_inv;
                Some(tw)
            })
            .collect::<Vec<_>>();

        let elements = (0..n)
            .scan(F::one(), |w, _| {
                let tw = *w;
                *w *= g;
                Some(tw)
            })
            .collect::<Vec<_>>();

        let bit_reverse = (0..n as u64)
            .filter_map(|i| {
                let r = i.reverse_bits() >> offset;
                (i < r).then_some((i as usize, r as usize))
            })
            .collect::<Vec<_>>();

        Fft {
            n,
            twiddle_factors,
            inv_twiddle_factors,
            cosets,
            inv_cosets,
            n_inv: F::from(n as u64).invert().unwrap(),
            bit_reverse,
            elements,
        }
    }

    /// polynomial degree
    pub fn size(&self) -> usize {
        self.n
    }

    /// nth unity of root
    pub fn generator(&self) -> F {
        self.twiddle_factors[1]
    }

    /// perform discrete fourier transform
    pub fn dft(&self, coeffs: &mut Polynomial<F>) {
        self.prepare_fft(coeffs);
        classic_fft_arithmetic(&mut coeffs.0, self.n, 1, &self.twiddle_factors)
    }

    /// perform classic inverse discrete fourier transform
    pub fn idft(&self, coeffs: &mut Polynomial<F>) {
        self.prepare_fft(coeffs);
        classic_fft_arithmetic(&mut coeffs.0, self.n, 1, &self.inv_twiddle_factors);
        coeffs.0.iter_mut().for_each(|coeff| *coeff *= self.n_inv)
    }

    /// perform discrete fourier transform on coset
    pub fn coset_dft(&self, coeffs: &mut Polynomial<F>) {
        coeffs
            .0
            .iter_mut()
            .zip(self.cosets.iter())
            .for_each(|(coeff, coset)| *coeff *= *coset);
        self.dft(coeffs)
    }

    /// perform discrete fourier transform on coset
    pub fn coset_idft(&self, coeffs: &mut Polynomial<F>) {
        self.idft(coeffs);
        coeffs
            .0
            .iter_mut()
            .zip(self.inv_cosets.iter())
            .for_each(|(coeff, inv_coset)| *coeff *= *inv_coset)
    }

    /// resize polynomial and bit reverse swap
    fn prepare_fft(&self, coeffs: &mut Polynomial<F>) {
        coeffs.0.resize(self.n, F::zero());
        self.bit_reverse
            .iter()
            .for_each(|(i, ri)| coeffs.0.swap(*ri, *i));
    }

    /// polynomial multiplication
    pub fn poly_mul(&self, mut rhs: Polynomial<F>, mut lhs: Polynomial<F>) -> Polynomial<F> {
        self.dft(&mut rhs);
        self.dft(&mut lhs);
        let mut mul_poly = Polynomial::new(
            rhs.0
                .iter()
                .zip(lhs.0.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        );
        self.idft(&mut mul_poly);
        mul_poly
    }
}

// classic fft using divide and conquer algorithm
fn classic_fft_arithmetic<F: FftField>(
    coeffs: &mut [F],
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

// butterfly arithmetic polynomial evaluation
fn butterfly_arithmetic<F: FftField>(
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
    use crate::poly::Polynomial;

    use super::Fft;
    use bls_12_381::Fr;
    use rand_core::OsRng;
    use zkstd::behave::{Group, PrimeField};
    use zkstd::common::Vec;

    fn arb_poly(k: u32) -> Vec<Fr> {
        (0..(1 << k))
            .map(|_| Fr::random(OsRng))
            .collect::<Vec<Fr>>()
    }

    fn naive_multiply<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
        assert_eq!(a.len(), b.len());
        let mut c = vec![F::zero(); a.len() + b.len()];
        a.iter().enumerate().for_each(|(i_a, coeff_a)| {
            b.iter().enumerate().for_each(|(i_b, coeff_b)| {
                c[i_a + i_b] += *coeff_a * *coeff_b;
            })
        });
        c
    }

    fn point_mutiply<F: PrimeField>(a: Polynomial<F>, b: Polynomial<F>) -> Polynomial<F> {
        assert_eq!(a.0.len(), b.0.len());
        Polynomial(
            a.0.iter()
                .zip(b.0.iter())
                .map(|(coeff_a, coeff_b)| *coeff_a * *coeff_b)
                .collect::<Vec<F>>(),
        )
    }

    #[test]
    fn fft_transformation_test() {
        let coeffs = arb_poly(10);
        let mut poly_a = Polynomial(coeffs);
        let poly_b = poly_a.clone();
        let classic_fft = Fft::new(10);

        classic_fft.dft(&mut poly_a);
        classic_fft.idft(&mut poly_a);

        assert_eq!(poly_a, poly_b)
    }

    #[test]
    fn fft_multiplication_test() {
        let coeffs_a = arb_poly(4);
        let coeffs_b = arb_poly(4);
        let fft = Fft::new(5);
        let poly_c = coeffs_a.clone();
        let poly_d = coeffs_b.clone();
        let mut poly_a = Polynomial(coeffs_a);
        let mut poly_b = Polynomial(coeffs_b);
        let poly_g = poly_a.clone();
        let poly_h = poly_b.clone();

        let poly_e = Polynomial(naive_multiply(poly_c, poly_d));

        fft.dft(&mut poly_a);
        fft.dft(&mut poly_b);
        let mut poly_f = point_mutiply(poly_a, poly_b);
        fft.idft(&mut poly_f);

        let poly_i = fft.poly_mul(poly_g, poly_h);

        assert_eq!(poly_e, poly_f);
        assert_eq!(poly_e, poly_i)
    }
}
