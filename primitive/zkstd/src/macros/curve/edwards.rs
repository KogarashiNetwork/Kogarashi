mod group;

pub use group::*;

/// Twisted Edwards curve group operation macro
#[macro_export]
macro_rules! twisted_edwards_curve_operation {
    ($scalar:ident, $range:ident, $d:ident, $affine:ident, $extended:ident, $x:ident, $y:ident, $t:ident) => {
        use zkstd::common::*;
        use zkstd::common::*;

        twisted_edwards_affine_group_operation!($affine, $extended, $range, $scalar, $x, $y);
        twisted_edwards_extend_group_operation!($affine, $extended, $range, $scalar, $x, $y, $t);
        mixed_curve_operations!($affine, $extended);

        impl ParityCmp for $affine {}
        impl ParityCmp for $extended {}
        impl Basic for $affine {}
        impl Basic for $extended {}
        impl ParallelCmp for $affine {}
        impl ParallelCmp for $extended {}

        impl TwistedEdwardsCurve for $affine {
            // d param
            const PARAM_D: $range = $d;
        }

        impl CurveAffine for $affine {
            fn to_extended(self) -> Self::Extended {
                Self::Extended {
                    x: self.x,
                    y: self.y,
                    t: self.x * self.y,
                    z: Self::Range::one(),
                }
            }

            fn to_raw_bytes(self) -> Vec<u8> {
                self.to_bytes().to_vec()
            }
        }

        impl TwistedEdwardsAffine for $affine {
            type Projective = $extended;
            fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self {
                Self { x, y }
            }

            fn new_extended(
                x: Self::Range,
                y: Self::Range,
                t: Self::Range,
                z: Self::Range,
            ) -> Self::Extended {
                Self::Extended { x, y, t, z }
            }
        }

        impl TwistedEdwardsCurve for $extended {
            // d param
            const PARAM_D: $range = $d;
        }

        impl CurveExtended for $extended {
            fn get_z(&self) -> Self::Range {
                self.z
            }

            fn to_affine(self) -> Self::Affine {
                let z_inv = self.z.invert().unwrap();
                Self::Affine {
                    x: self.x * z_inv,
                    y: self.y * z_inv,
                }
            }
        }

        impl TwistedEdwardsExtended for $extended {
            fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self {
                Self { x, y, t, z }
            }

            fn get_t(&self) -> Self::Range {
                self.t
            }
        }

        impl From<$extended> for $affine {
            fn from(p: $extended) -> $affine {
                p.to_affine()
            }
        }

        impl From<$affine> for $extended {
            fn from(p: $affine) -> $extended {
                p.to_extended()
            }
        }
    };
}

pub use twisted_edwards_curve_operation;
