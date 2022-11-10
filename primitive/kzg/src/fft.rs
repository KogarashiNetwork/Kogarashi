use crate::poly::Polynomial;
use zero_crypto::common::*;

#[derive(Clone, Debug)]
pub struct Fft<F: FftField> {
    // polynomial degree 2^k
    k: u32,
    // generator of order 2^{k - 1} multiplicative group used as twiddle factors
    twiddle_factors: Vec<F>,
    // multiplicative group generator inverse
    inv_twiddle_factors: Vec<F>,
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
                *w *= inv;
                Some(tw)
            })
            .collect::<Vec<_>>();

        Fft {
            k,
            twiddle_factors,
            inv_twiddle_factors,
        }
    }
}
