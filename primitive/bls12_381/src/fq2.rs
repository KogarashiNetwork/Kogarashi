use crate::fq::Fq;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::dress::extention_field::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fq2(pub(crate) [Fq; 2]);

extention_field_operation!(Fq2, Fq);
