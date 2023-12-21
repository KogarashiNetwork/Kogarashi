use crate::poly::{Coefficients, PointsValue};
#[cfg(feature = "std")]
use rayon::join;
use zkstd::common::{FftField, Vec};

/// fft construction using n th root of unity supports polynomial operation less than n degree
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
}

// SBP-M1 review: use safe math operations
impl<F: FftField> Fft<F> {
    pub(crate) fn new(k: usize) -> Self {
        assert!(k >= 1);
        let n = 1 << k;
        let half_n = n >> 1;
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
        let cosets = (0..n)
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

        let bit_reverse = (0..n as u64)
            .filter_map(|i| {
                let r = i.reverse_bits() >> offset;
                (i < r).then_some((i as usize, r as usize))
            })
            .collect::<Vec<_>>();

        Self {
            n,
            twiddle_factors,
            inv_twiddle_factors,
            cosets,
            inv_cosets,
            n_inv: F::from(n as u64).invert().unwrap(),
            bit_reverse,
        }
    }

    /// perform discrete fourier transform
    pub(crate) fn dft(&self, coeffs: Coefficients<F>) -> PointsValue<F> {
        let mut evals = coeffs.0;
        self.prepare_fft(&mut evals);
        classic_fft_arithmetic(&mut evals, self.n, 1, &self.twiddle_factors);
        PointsValue::new(evals.clone())
    }

    /// perform classic inverse discrete fourier transform
    pub(crate) fn idft(&self, points: PointsValue<F>) -> Coefficients<F> {
        let mut coeffs = points.0;
        self.prepare_fft(&mut coeffs);
        classic_fft_arithmetic(&mut coeffs, self.n, 1, &self.inv_twiddle_factors);
        coeffs.iter_mut().for_each(|coeff| *coeff *= self.n_inv);
        Coefficients::new(coeffs.clone())
    }

    /// perform discrete fourier transform on coset
    pub(crate) fn coset_dft(&self, mut coeffs: Coefficients<F>) -> PointsValue<F> {
        coeffs
            .0
            .iter_mut()
            .zip(self.cosets.iter())
            .for_each(|(coeff, coset)| *coeff *= *coset);
        self.dft(coeffs)
    }

    /// perform discrete fourier transform on coset
    pub(crate) fn coset_idft(&self, points: PointsValue<F>) -> Coefficients<F> {
        let mut points = self.idft(points);
        points
            .0
            .iter_mut()
            .zip(self.inv_cosets.iter())
            .for_each(|(coeff, inv_coset)| *coeff *= *inv_coset);
        Coefficients::new(points.0)
    }

    /// This evaluates t(tau) for this domain, which is
    /// tau^m - 1 for these radix-2 domains.
    pub(crate) fn z(&self, tau: &F) -> F {
        let mut tmp = tau.pow(self.n as u64);
        tmp.sub_assign(&F::one());

        tmp
    }

    /// This evaluates t(tau) for this domain, which is
    /// tau^m - 1 for these radix-2 domains.
    pub(crate) fn z_on_coset(&self) -> F {
        let mut tmp = F::MULTIPLICATIVE_GENERATOR.pow(self.n as u64);
        tmp.sub_assign(&F::one());

        tmp
    }

    /// The target polynomial is the zero polynomial in our
    /// evaluation domain, so we must perform division over
    /// a coset.
    pub(crate) fn divide_by_z_on_coset(&self, points: PointsValue<F>) -> PointsValue<F> {
        let i = self.z_on_coset().invert().unwrap();

        PointsValue(points.0.into_iter().map(|v| v * i).collect())
    }

    /// resize polynomial and bit reverse swap
    fn prepare_fft(&self, coeffs: &mut Vec<F>) {
        coeffs.resize(self.n, F::zero());
        self.bit_reverse
            .iter()
            .for_each(|(i, ri)| coeffs.swap(*ri, *i));
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
        #[cfg(feature = "std")]
        join(
            || classic_fft_arithmetic(left, n / 2, twiddle_chunk * 2, twiddles),
            || classic_fft_arithmetic(right, n / 2, twiddle_chunk * 2, twiddles),
        );
        #[cfg(not(feature = "std"))]
        {
            // TODO: recursion is quite inefficient when not parallel
            classic_fft_arithmetic(left, n / 2, twiddle_chunk * 2, twiddles);
            classic_fft_arithmetic(right, n / 2, twiddle_chunk * 2, twiddles);
        };
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
    use super::Fft;
    use crate::poly::{Coefficients, PointsValue};

    use bn_254::Fr;
    use zkstd::common::{vec, Group, OsRng, PrimeField, Vec};

    fn arb_poly(k: u32) -> Vec<Fr> {
        let mut rng = OsRng;
        (0..(1 << k))
            .map(|_| Fr::random(&mut rng))
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

    #[test]
    fn fft_transformation_test() {
        let coeffs = arb_poly(10);
        let poly_a = Coefficients(coeffs);
        let poly_b = poly_a.clone();
        let classic_fft = Fft::new(10);

        let evals_a = classic_fft.dft(poly_a);
        let poly_a = classic_fft.idft(evals_a);

        assert_eq!(poly_a, poly_b)
    }

    #[test]
    fn fft_multiplication_test() {
        let coeffs_a = arb_poly(4);
        let coeffs_b = arb_poly(4);
        let fft = Fft::new(5);
        let poly_c = coeffs_a.clone();
        let poly_d = coeffs_b.clone();
        let poly_a = Coefficients(coeffs_a);
        let poly_b = Coefficients(coeffs_b);
        let poly_g = poly_a.clone();
        let poly_h = poly_b.clone();

        let poly_e = Coefficients::new(naive_multiply(poly_c, poly_d));

        let evals_a = fft.dft(poly_a);
        let evals_b = fft.dft(poly_b);
        let poly_f = &evals_a * &evals_b;
        let poly_f = fft.idft(poly_f);

        let rhs = fft.dft(poly_g);
        let lhs = fft.dft(poly_h);
        let mul_poly = PointsValue::new(
            rhs.0
                .iter()
                .zip(lhs.0.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        );
        let poly_i = fft.idft(mul_poly);

        assert_eq!(poly_e, poly_f);
        assert_eq!(poly_e, poly_i)
    }
}
