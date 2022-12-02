use crate::fq2::Fq2;
use zero_crypto::dress::extention_field::*;

const LIMBS_LENGTH: usize = 3;

construct_extention_field!(Fq6, Fq2, LIMBS_LENGTH);

extention_field_built_in!(Fq6);

const_extention_field_operation!(Fq6, Fq2, LIMBS_LENGTH);
