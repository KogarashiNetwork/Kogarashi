use crate::fq2::Fq2;
use zero_crypto::common::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fq6(pub(crate) [Fq2; 3]);

const ZERO: Fq6 = Fq6([Fq2::zero(); 3]);
