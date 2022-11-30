use zero_crypto::behave::Commitment;

pub struct Witness<C: Commitment> {
    pub(crate) s_eval: C::G1Affine,
    pub(crate) a_eval: C::G1Affine,
    pub(crate) q_eval: C::G1Affine,
    pub(crate) denominator: C::G2Affine,
}

impl<C: Commitment> Witness<C> {
    pub fn verify(self) -> bool {
        false
    }
}
