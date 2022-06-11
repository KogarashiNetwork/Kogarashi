use crate::entity::Fr;

/// The affine form of coordinate
#[derive(Debug)]
pub struct Affine {
    x: Fr,
    y: Fr,
}

impl Affine {
    fn is_on_curve(&self) -> bool {
        true
    }
}
