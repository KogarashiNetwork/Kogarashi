use crate::arithmetic::coordinate::utils::*;

pub fn add_point(
    lhs: ProjectiveCoordinate<[u64; 6]>,
    rhs: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    rhs
}

pub fn double_point(
    rhs: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    rhs
}

pub fn scalar_point(
    mut base: ProjectiveCoordinate<[u64; 6]>,
    scalar: [u64; 6],
    mut identity: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    base
}
