use zkstd::common::PrimeField;

// Computes f(f-1)(f-2)(f-3)
pub(crate) fn delta<F: PrimeField>(f: F) -> F {
    let f_1 = f - F::one();
    let f_2 = f - F::from(2);
    let f_3 = f - F::from(3);
    f * f_1 * f_2 * f_3
}

// The identity we want to check is q_logic * A = 0
// A = B + E
// B = q_c * [9c - 3(a+b)]
// E = 3(a+b+c) - 2F
// F = w[w(4w - 18(a+b) + 81) + 18(a^2 + b^2) - 81(a+b) + 83]
#[allow(non_snake_case)]
pub(crate) fn delta_xor_and<F: PrimeField>(a: &F, b: &F, w: &F, c: &F, q_c: &F) -> F {
    let nine = F::from(9);
    let two = F::from(2);
    let three = F::from(3);
    let four = F::from(4);
    let eighteen = F::from(18);
    let eighty_one = F::from(81);
    let eighty_three = F::from(83);

    let F = *w
        * (*w * (four * w - eighteen * (*a + b) + eighty_one)
            + eighteen * (a.square() + b.square())
            - eighty_one * (*a + b)
            + eighty_three);
    let E = three * (*a + b + c) - (two * F);
    let B = *q_c * ((nine * c) - three * (*a + b));
    B + E
}

pub fn extract_bit<F: PrimeField>(curr_acc: &F, next_acc: &F) -> F {
    // Next - 2 * current
    *next_acc - *curr_acc - *curr_acc
}

// Ensures that the bit is either +1, -1 or 0
pub fn check_bit_consistency<F: PrimeField>(bit: F) -> F {
    let one = F::one();
    bit * (bit - one) * (bit + one)
}
