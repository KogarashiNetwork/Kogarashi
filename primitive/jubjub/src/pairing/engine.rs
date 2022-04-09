use super::cordinate::CurveAffine;
use super::field::{Field, PrimeField};

/// `Fr` and `Curve` necessary for pairing
pub trait Engine {
    type Fr: PrimeField;

    type G1Affine: CurveAffine;

    type G2Affine: CurveAffine;

    type Fq: PrimeField;

    type Fqk: Field;

    fn pairing() -> Self::Fqk;
}
