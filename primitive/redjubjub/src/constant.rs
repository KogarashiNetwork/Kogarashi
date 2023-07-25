use bls_12_381::Fr;
use jub_jub::JubjubAffine;

pub(crate) const SAPLING_PERSONAL: &[u8; 16] = b"Zcash_RedJubjubH";

pub(crate) const KOGARASHI_PERSONAL: &[u8; 16] = b"Kogarashi_FSeedH";

const SAPLING_BASE_POINT_X: Fr = Fr::to_mont_form([
    0x47bf46920a95a753,
    0xd5b9a7d3ef8e2827,
    0xd418a7ff26753b6a,
    0x0926d4f32059c712,
]);

const SAPLING_BASE_POINT_Y: Fr = Fr::to_mont_form([
    0x305632adaaf2b530,
    0x6d65674dcedbddbc,
    0x53bb37d0c21cfd05,
    0x57a1019e6de9b675,
]);

pub(crate) const SAPLING_BASE_POINT: JubjubAffine =
    JubjubAffine::from_raw_unchecked(SAPLING_BASE_POINT_X, SAPLING_BASE_POINT_Y);

pub(crate) const SAPLING_REDJUBJUB_COFACTOR: Fr = Fr::to_mont_form([
    0x0000000000000008,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
]);
