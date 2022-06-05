use crate::entity::Fr;

/// The projective form of coordinate
pub(crate) struct ProjectiveCoordinate {}

/// The affine form of coordinate
pub(crate) struct AffineCoordinate {}

/// Extended twisted edwards coordinate
#[derive(Debug)]
pub(crate) struct Coordinate {
    x: Fr,
    y: Fr,
    z: Fr,
    t: Fr,
}

impl Coordinate {
    pub fn add(&self, other: Self) {}

    pub fn double(&self) {}
}
