mod group;
mod test;

pub use group::*;
pub use test::*;

/// Weierstrass standard curve operation macro
#[macro_export]
macro_rules! weierstrass_curve_operation {
    ($scalar:ident, $range:ident, $b:ident, $b3:ident, $affine:ident, $projective:ident, $x:ident, $y:ident) => {
        affine_group_operation!($affine, $projective, $range, $scalar, $x, $y, $b, $b3);
        projective_group_operation!($affine, $projective, $range, $scalar, $x, $y, $b, $b3);
        mixed_curve_operations!($affine, $projective);

        impl ParityCmp for $affine {}
        impl ParityCmp for $projective {}
        impl Basic for $affine {}
        impl Basic for $projective {}
        impl ParallelCmp for $affine {}
        impl ParallelCmp for $projective {}

        impl BNAffine for $affine {
            type Extended = $projective;

            fn to_extended(self) -> $projective {
                if self.is_identity() {
                    $projective::ADDITIVE_IDENTITY
                } else {
                    $projective {
                        x: self.x,
                        y: self.y,
                        z: Self::Base::one(),
                    }
                }
            }

            fn double(self) -> $projective {
                double_affine_point(self)
            }
        }

        impl BNProjective for $projective {
            type Affine = $affine;

            fn new(x: Self::Base, y: Self::Base, z: Self::Base) -> Self {
                Self { x, y, z }
            }

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

            fn get_z(&self) -> Self::Base {
                self.z
            }

            fn double(self) -> $projective {
                double_projective_point(self)
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
