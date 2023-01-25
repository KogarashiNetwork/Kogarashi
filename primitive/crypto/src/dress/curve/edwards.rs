mod group;

pub use group::{
    twisted_edwards_affine_group_operation, twisted_edwards_curve_arithmetic_extension,
    twisted_edwards_extend_group_operation,
};

#[macro_export]
macro_rules! twisted_edwards_curve_operation {
    ($scalar:ident, $range:ident, $d:ident, $affine:ident, $extend:ident, $x:ident, $y:ident, $t:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        twisted_edwards_affine_group_operation!($affine, $extend, $range, $scalar, $x, $y);
        twisted_edwards_extend_group_operation!($affine, $extend, $range, $scalar, $x, $y, $t);
        twisted_edwards_mixed_curve_operation!($affine, $extend);

        impl ParityCmp for $affine {}
        impl ParityCmp for $extend {}
        impl Basic for $affine {}
        impl Basic for $extend {}

        impl Curve for $affine {
            type Range = $scalar;

            type Scalar = $scalar;

            const PARAM_A: $scalar = $scalar::one();

            fn is_identity(self) -> bool {
                self.x.is_zero() && self.y == $scalar::one()
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

        impl TwistedEdwardsAffine for $affine {
            type CurveExtend = $extend;

            fn double(self) -> Self::CurveExtend {
                double_point(self.to_extend())
            }

            fn to_extend(self) -> Self::CurveExtend {
                Self::CurveExtend {
                    x: self.x,
                    y: self.y,
                    t: self.x * self.y,
                    z: Self::Range::one(),
                }
            }
        }

        impl From<$extend> for $affine {
            fn from(p: $extend) -> $affine {
                p.to_affine()
            }
        }

        impl Affine for $affine {}

        impl Curve for $extend {
            type Range = $scalar;

            type Scalar = $scalar;

            const PARAM_A: $scalar = $scalar::one();

            fn is_identity(self) -> bool {
                self.x == $scalar::zero() && self.y == $scalar::one()
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

        impl TwistedEdwardsCurve for $extend {
            // d param
            const PARAM_D: $range = $d;
        }

        impl CurveExtend for $extend {
            type Affine = $affine;

            fn double(self) -> Self {
                double_point(self)
            }

            fn to_affine(self) -> Self::Affine {
                let z_inv = self.z.invert().unwrap();
                Self::Affine {
                    x: self.x * z_inv,
                    y: self.y * z_inv,
                }
            }
        }

        impl Extended for $extend {
            fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self {
                Self { x, y, t, z }
            }

            fn get_t(&self) -> Self::Range {
                self.t
            }

            fn get_z(&self) -> Self::Range {
                self.z
            }
        }

        impl From<$affine> for $extend {
            fn from(p: $affine) -> $extend {
                p.to_extend()
            }
        }
    };
}

#[macro_export]
macro_rules! twisted_edwards_mixed_curve_operation {
    ($affine:ident, $extend:ident) => {
        impl Add<$extend> for $affine {
            type Output = $extend;

            fn add(self, rhs: $extend) -> $extend {
                add_point(self.to_extend(), rhs)
            }
        }

        impl Sub<$extend> for $affine {
            type Output = $extend;

            fn sub(self, rhs: $extend) -> $extend {
                add_point(self.to_extend(), -rhs)
            }
        }

        impl Add<$affine> for $extend {
            type Output = $extend;

            fn add(self, rhs: $affine) -> $extend {
                add_point(self, rhs.to_extend())
            }
        }

        impl Sub<$affine> for $extend {
            type Output = $extend;

            fn sub(self, rhs: $affine) -> $extend {
                add_point(self, -rhs.to_extend())
            }
        }

        impl AddAssign<$affine> for $extend {
            fn add_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, rhs.to_extend())
            }
        }

        impl SubAssign<$affine> for $extend {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, -rhs.to_extend())
            }
        }
    };
}

pub use {twisted_edwards_curve_operation, twisted_edwards_mixed_curve_operation};
