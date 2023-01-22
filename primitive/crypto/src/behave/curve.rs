// This trait resresents elliptic curve and its scalar field
mod basic;
mod edwards;
mod weierstrass;

pub use basic::{Affine, Curve, CurveExtend};
pub use edwards::{Extended, TwistedEdwardsAffine, TwistedEdwardsCurve};
pub use weierstrass::{Projective, WeierstrassAffine, WeierstrassCurve};
