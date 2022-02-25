#[cfg(test)]
mod fft_tests {
    use zero_jubjub::Fr;

    #[test]
    fn fft_test() {
        let mut a_coeffs = vec![];
        let mut b_coeffs = vec![];
        let exponent_of_two = 6;
        let poly_degree = (1u64 << exponent_of_two) as usize;
        let mut naive_result = Vec::<Fr>::with_capacity(poly_degree * 2);
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
    }
}
