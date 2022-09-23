use parity_scale_codec::alloc::vec::Vec;
use zero_jubjub::{coordinate::Affine, fr};

pub(crate) struct Kzg {
    k: u32,
    g1_projective: Vec<Affine>,
    g2_projective: Vec<Affine>,
}
