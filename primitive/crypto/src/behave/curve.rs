// This trait resresents elliptic curve and its scalar field
mod basic;
mod edwards;
mod weierstrass;

pub use basic::{Affine, Curve, CurveExtended};
pub use edwards::{TwistedEdwardsAffine, TwistedEdwardsCurve, TwistedEdwardsExtended};
pub use weierstrass::{Projective, WeierstrassAffine, WeierstrassCurve};
