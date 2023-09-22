// This trait resresents elliptic curve and its scalar field
mod basic;
mod edwards;
mod weierstrass;

pub use basic::{CurveAffine, CurveExtended};
pub use edwards::{TwistedEdwardsAffine, TwistedEdwardsCurve, TwistedEdwardsExtended};
pub use weierstrass::{WeierstrassAffine, WeierstrassCurve, WeierstrassProjective};
