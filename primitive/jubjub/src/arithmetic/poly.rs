use crate::fr::Fr;
use parity_scale_codec::alloc::vec::Vec;

pub struct Polynomial {
    values: Vec<Fr>,
}
