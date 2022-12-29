use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use zero_crypto::dress::extension_field::*;
use zero_crypto::dress::pairing::{bls12_range_field_pairing, peculiar_extension_field_operation};

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENTION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(Fq12, Fq2, G1Affine, PairingCoeff);

// non common extension operation
peculiar_extension_field_operation!(Fq2, Fq6, Fq12);
