use crate::fq2::Fq2;
use zero_crypto::dress::extention_field::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fq6(pub(crate) [Fq2; 3]);

const ZERO: Fq6 = Fq6([Fq2::zero(); 3]);

const ONE: [Fq2; 3] = [Fq2::one(), Fq2::zero(), Fq2::zero()];

const LIMBS_LENGTH: usize = 3;

extention_field_built_in!(Fq6);

const_extention_field_operation!(Fq6, Fq2, LIMBS_LENGTH, ONE);
