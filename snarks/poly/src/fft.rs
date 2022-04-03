use sp_std::{vec, vec::Vec};
use zero_jubjub::Fr;

pub struct Fft {
    k: usize,
    n: usize,
    twiddles: Vec<Fr>,
    reversed_indexes: Vec<usize>,
}

impl Fft {
    pub fn new(k: usize, omega: Fr) -> Self {
        let n = 1 << k;
        let mut counter = 2;
        let mut twiddles = Vec::with_capacity(n / 2);
        let mut reversed_indexes = vec![0; n];

        let mut twiddle = Fr::one();
        for _ in 0..n / 2 {
            twiddles.push(twiddle);
            twiddle *= omega;
        }

        if k % 2 != 0 {
            reversed_indexes[0] = 0;
            reversed_indexes[1] = 1;
        } else {
            reversed_indexes[0] = 0;
            reversed_indexes[1] = 2;
            reversed_indexes[2] = 1;
            reversed_indexes[3] = 3;
            counter *= 2;
        }

        // calculate bit reverse indexes
        // Todo: two arrays, swap pair
        while counter != n {
            for i in 0..counter {
                reversed_indexes[i] *= 4;
                reversed_indexes[i + counter] = reversed_indexes[i] + 2;
                reversed_indexes[i + 2 * counter] = reversed_indexes[i] + 1;
                reversed_indexes[i + 3 * counter] = reversed_indexes[i] + 3;
            }
            counter *= 4;
        }

        Fft {
            k,
            n,
            twiddles,
            reversed_indexes,
        }
    }

    fn bit_reverse(&self, coeffs: &mut [Fr]) {
        let tmp = coeffs.to_vec();
        for (i, coeff) in tmp.iter().enumerate() {
            coeffs[self.reversed_indexes[i]] = *coeff;
        }
    }

    pub fn fft(&self, coeffs: &mut [Fr]) {
        self.bit_reverse(coeffs);
    }
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
