mod group;
mod test;

pub use group::*;
pub use test::*;

/// Weierstrass standard curve operation macro
#[macro_export]
macro_rules! weierstrass_curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $x:ident, $y:ident) => {
        use zkstd::behave::*;
        use zkstd::common::*;

        affine_group_operation!($affine, $projective, $range, $scalar, $x, $y);
        projective_group_operation!($affine, $projective, $range, $scalar, $x, $y);
        mixed_curve_operations!($affine, $projective);

        impl ParityCmp for $affine {}
        impl ParityCmp for $projective {}
        impl Basic for $affine {}
        impl Basic for $projective {}

        impl Curve for $affine {
            type Range = $range;

            const PARAM_A: $range = $a;

            fn double(self) -> $projective {
                double_point(self.to_extended())
            }

            fn is_on_curve(self) -> bool {
                if self.is_infinity {
                    true
                } else {
                    self.y.square() == self.x.square() * self.x + Self::PARAM_B
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl WeierstrassCurve for $affine {
            const PARAM_B: $range = $b;
        }

        impl Affine for $affine {
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
        }

        impl WeierstrassAffine for $affine {}

        impl Curve for $projective {
            type Range = $range;

            const PARAM_A: $range = $a;

            fn double(self) -> Self {
                double_point(self)
            }

            fn is_on_curve(self) -> bool {
                if self.is_identity() {
                    true
                } else {
                    self.y.square() * self.z
                        == self.x.square() * self.x + Self::PARAM_B * self.z.square() * self.z
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl WeierstrassCurve for $projective {
            const PARAM_B: $range = $b;
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

        impl Projective for $projective {
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
