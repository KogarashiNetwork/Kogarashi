use zkstd::common::{Decode, Encode, PrimeField};

/// Subset of all of the evaluations
#[derive(Debug, Eq, PartialEq, Clone, Default, Decode, Encode)]
pub struct Evaluations<F: PrimeField> {
    // left wire witness polynomial evaluation at `z`
    pub a_eval: F,
    // right wire witness polynomial evaluation at `z`
    pub b_eval: F,
    // output wire witness polynomial evaluation at `z`
    pub c_eval: F,
    // fourth wire witness polynomial evaluation at `z`
    pub d_eval: F,
    pub a_next_eval: F,
    pub b_next_eval: F,
    // fourth wire witness polynomial evaluation at `z * root of unity`
    pub d_next_eval: F,
    // Evaluation of the arithmetic selector polynomial at `z`
    pub q_arith_eval: F,
    pub q_c_eval: F,
    pub q_l_eval: F,
    pub q_r_eval: F,
    // left sigma polynomial evaluation at `z`
    pub s_sigma_1_eval: F,
    // right sigma polynomial evaluation at `z`
    pub s_sigma_2_eval: F,
    // out sigma polynomial evaluation at `z`
    pub s_sigma_3_eval: F,
    // linearization sigma polynomial evaluation at `z`
    pub r_poly_eval: F,
    // permutation polynomial evaluation at `z * root of unity`
    pub perm_eval: F,
}
