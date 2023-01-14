// This trait resresents elliptic curve and its scalar field
mod basic;
mod weierstrass;

pub use basic::{Affine, Curve, CurveExtend};
pub use weierstrass::{Projective, WeierstrassAffine, WeierstrassCurve};
