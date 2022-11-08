use crate::behave::{Curve, PrimeField, Projective};

pub struct ProjectiveCoordinate<E: Curve> {
    pub(crate) x: E::ScalarField,
    pub(crate) y: E::ScalarField,
    pub(crate) z: E::ScalarField,
}
