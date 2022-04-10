use sp_std::{vec, vec::Vec};
use zero_jubjub::Fr;

pub struct Butterfly {
    k: usize,
    n: usize,
    twiddles: Vec<Fr>,
}

impl Butterfly {
    pub fn new(k: usize, omega: Fr) -> Self {
        let n = 1 << k;
        let mut twiddles = vec![Fr::zero(); n / 2];

        let mut twiddle = Fr::one();
        twiddles.iter_mut().for_each(|value| {
            *value = twiddle;
            twiddle *= omega;
        });

        Butterfly { k, n, twiddles }
    }
}

pub fn bit_reverse_indexes(k: usize) -> Vec<(usize, usize)> {
    let n = (1 << k) as usize;
    let mut indexes = Vec::with_capacity(n / 2);
    for i in 0..n {
        let ri = bit_reverse(i, k);
        if i < ri {
            indexes.push((ri, i));
        }
    }
    indexes
}

fn bit_reverse(mut i: usize, k: usize) -> usize {
    let mut r = 0;
    for _ in 0..k {
        r = (r << 1) | (i & 1);
        i >>= 1;
    }
    r
}

#[cfg(test)]
mod fft_tests {
    use super::*;

    #[test]
    fn fft_test() {
        let mut a_coeffs = vec![];
        let mut b_coeffs = vec![];
        let exponent_of_two = 6;
        let poly_degree = (1u64 << exponent_of_two) as usize;
        let mut naive_result = vec![Fr::zero(); poly_degree * 2];
        for _ in 0..poly_degree {
            let rng = &mut rand::thread_rng();
            a_coeffs.push(Fr::random(rng));
        }
        for _ in 0..poly_degree {
            let rng = &mut rand::thread_rng();
            b_coeffs.push(Fr::random(rng));
        }
        for a in 0..poly_degree {
            for b in 0..poly_degree {
                naive_result[a + b] = a_coeffs[a] * b_coeffs[b];
            }
        }
        assert_eq!(naive_result.len(), poly_degree * 2)
    }
}
