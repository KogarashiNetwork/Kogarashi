mod group;
mod test;

pub use group::*;
pub use test::*;

/// Weierstrass standard curve operation macro
#[macro_export]
macro_rules! weierstrass_curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $b3:ident, $affine:ident, $projective:ident, $x:ident, $y:ident) => {
        use zkstd::common::*;
        use zkstd::common::*;

        affine_group_operation!($affine, $projective, $range, $scalar, $x, $y, $a);
        projective_group_operation!($affine, $projective, $range, $scalar, $x, $y, $a);
        mixed_curve_operations!($affine, $projective);

        impl ParityCmp for $affine {}
        impl ParityCmp for $projective {}
        impl Basic for $affine {}
        impl Basic for $projective {}
        impl ParallelCmp for $affine {}
        impl ParallelCmp for $projective {}

        impl WeierstrassCurve for $affine {
            const PARAM_B: $range = $b;
            const PARAM_3B: $range = $b3;
        }

        impl CurveAffine for $affine {
            fn to_extended(self) -> $projective {
                if self.is_identity() {
                    $projective::ADDITIVE_IDENTITY
                } else {
                    $projective {
                        x: self.x,
                        y: self.y,
                        z: Self::Range::one(),
                    }
                }
            }

            fn to_raw_bytes(self) -> Vec<u8> {
                self.to_bytes().to_vec()
            }
        }

        impl WeierstrassAffine for $affine {
            type Projective = $projective;

            fn to_projective(self) -> $projective {
                if self.is_identity() {
                    $projective::ADDITIVE_IDENTITY
                } else {
                    $projective {
                        x: self.x,
                        y: self.y,
                        z: Self::Range::one(),
                    }
                }
            }
        }

        impl WeierstrassCurve for $projective {
            const PARAM_B: $range = $b;
            const PARAM_3B: $range = $b3;
        }

        impl CurveExtended for $projective {
            fn to_affine(self) -> Self::Affine {
                match self.z.invert() {
                    Some(z_inv) => Self::Affine {
                        x: self.x * z_inv,
                        y: self.y * z_inv,
                        is_infinity: false,
                    },
                    None => Self::Affine::ADDITIVE_IDENTITY,
                }
            }

            fn get_z(&self) -> Self::Range {
                self.z
            }
        }

        impl WeierstrassProjective for $projective {
            fn new(x: Self::Range, y: Self::Range, z: Self::Range) -> Self {
                Self { x, y, z }
            }
        }

        impl From<$projective> for $affine {
            fn from(p: $projective) -> $affine {
                p.to_affine()
            }
        }

        impl From<$affine> for $projective {
            fn from(a: $affine) -> $projective {
                a.to_extended()
            }
        }
    };
}

pub use weierstrass_curve_operation;
