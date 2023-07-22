mod group;

pub use group::*;

#[macro_export]
macro_rules! twisted_edwards_curve_operation {
    ($scalar:ident, $range:ident, $d:ident, $affine:ident, $extended:ident, $x:ident, $y:ident, $t:ident) => {
        use zkstd::behave::*;
        use zkstd::common::*;

        twisted_edwards_affine_group_operation!($affine, $extended, $range, $scalar, $x, $y);
        twisted_edwards_extend_group_operation!($affine, $extended, $range, $scalar, $x, $y, $t);
        mixed_curve_operations!($affine, $extended);

        impl ParityCmp for $affine {}
        impl ParityCmp for $extended {}
        impl Basic for $affine {}
        impl Basic for $extended {}

        impl Curve for $affine {
            type Range = $scalar;

            const PARAM_A: $scalar = $scalar::one();

            fn double(self) -> Self::Extended {
                double_point(self.to_extended())
            }

            fn is_on_curve(self) -> bool {
                if self.x.is_zero() {
                    true
                } else {
                    let xx = self.x.square();
                    let yy = self.y.square();
                    yy == $scalar::one() + Self::PARAM_D * xx * yy + xx
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl TwistedEdwardsCurve for $affine {
            // d param
            const PARAM_D: $range = $d;
        }

        impl Affine for $affine {
            fn to_extended(self) -> Self::Extended {
                Self::Extended {
                    x: self.x,
                    y: self.y,
                    t: self.x * self.y,
                    z: Self::Range::one(),
                }
            }
        }

        impl TwistedEdwardsAffine for $affine {
            fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self {
                Self { x, y }
            }
        }

        impl Curve for $extended {
            type Range = $scalar;

            const PARAM_A: $scalar = $scalar::one();

            fn double(self) -> Self {
                double_point(self)
            }

            fn is_on_curve(self) -> bool {
                if self.z.is_zero() {
                    true
                } else {
                    let affine = $affine::from(self);
                    affine.is_on_curve()
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
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

            fn batch_normalize<'a>(
                y: &'a mut [$extended],
            ) -> Box<dyn Iterator<Item = Self::Affine> + 'a> {
                Box::new(y.iter().map(|p| Self::Affine::from(*p)))
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
