use rayon::join;
use zero_crypto::behave::FftField;

// classic fft using divide and conquer algorithm
pub(crate) fn classic_fft_arithmetic<F: FftField>(
    coeffs: &mut [F],
    n: usize,
    twiddle_chunk: usize,
    twiddles: &Vec<F>,
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
pub fn butterfly_arithmetic<F: FftField>(
    left: &mut [F],
    right: &mut [F],
    twiddle_chunk: usize,
    twiddles: &Vec<F>,
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
