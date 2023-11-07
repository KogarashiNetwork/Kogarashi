// This trait resresents elliptic curve and its scalar field
mod edwards;
mod weierstrass;

pub use edwards::{TwistedEdwardsAffine, TwistedEdwardsCurve, TwistedEdwardsExtended};
pub use weierstrass::{WeierstrassAffine, WeierstrassCurve, WeierstrassProjective};
