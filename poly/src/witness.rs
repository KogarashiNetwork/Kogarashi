use zkstd::common::{Group, Pairing, PairingRange};

/// witness for Kate polynomial commitment
#[allow(dead_code)]
#[derive(Debug)]
pub struct Witness<P: Pairing> {
    // Original commitment, C = p(s)
    pub c_eval: P::G1Affine,
    // Quotient commitment, C = q(s)
    pub q_eval: P::G1Affine,
    // (s - a)_g2
    pub denominator: P::G2PairngRepr,
    // H
    pub h: P::G2PairngRepr,
}

impl<P: Pairing> Witness<P> {
    pub fn verify(self) -> bool {
        let pairing =
            P::multi_miller_loop(&[(self.c_eval, self.h), (self.q_eval, self.denominator)])
                .final_exp();

        pairing == <<P as Pairing>::PairingRange as PairingRange>::Gt::ADDITIVE_IDENTITY
    }
}
