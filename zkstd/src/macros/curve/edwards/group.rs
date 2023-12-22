#[macro_export]
macro_rules! twisted_edwards_affine_group_operation {
    ($affine:ident, $extended:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $d:ident) => {
        curve_arithmetic_extension!($affine, $scalar, $extended);
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Group for $affine {
            const ADDITIVE_GENERATOR: Self = Self { x: $x, y: $y };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
            };

            fn invert(self) -> Option<Self> {
                match self.x.is_zero() {
                    true => None,
                    false => Some(Self {
                        x: -self.x,
                        y: self.y,
                    }),
                }
            }

            fn random<R: RngCore>(rand: &mut R) -> $affine {
                (Self::ADDITIVE_GENERATOR * $scalar::random(rand)).into()
            }
        }

        impl CurveGroup for $affine {
            type Base = $range;

            fn from_x_and_y(x: $range, y: $range) -> $affine {
                $affine { x, y }
            }

            fn is_identity(&self) -> bool {
                self == &Self::ADDITIVE_IDENTITY
            }

            fn is_on_curve(self) -> bool {
                if self.x.is_zero() {
                    true
                } else {
                    let xx = self.x.square();
                    let yy = self.y.square();
                    yy == $range::one() + Self::PARAM_D * xx * yy + xx
                }
            }

            fn get_x(&self) -> Self::Base {
                self.x
            }

            fn get_y(&self) -> Self::Base {
                self.y
            }
        }

        impl TwistedEdwardsCurve for $affine {
            const PARAM_D: $range = $d;
            type Scalar = $scalar;
        }
    };
}

#[macro_export]
macro_rules! twisted_edwards_extend_group_operation {
    ($affine:ident, $extended:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $t:ident, $d:ident) => {
        curve_arithmetic_extension!($extended, $scalar, $extended);

        impl PartialEq for $extended {
            fn eq(&self, other: &Self) -> bool {
                self.x * other.z == other.x * self.z && self.y * &other.z == other.y * self.z
            }
        }

        impl Group for $extended {
            const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                t: $t,
                z: $range::one(),
            };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
                t: $range::zero(),
                z: $range::one(),
            };

            fn invert(self) -> Option<Self> {
                match self.z.is_zero() {
                    true => None,
                    false => Some(Self {
                        x: -self.x,
                        y: self.y,
                        t: -self.t,
                        z: self.z,
                    }),
                }
            }

            fn random<R: RngCore>(rand: &mut R) -> Self {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl CurveGroup for $extended {
            type Base = $range;

            fn from_x_and_y(x: $range, y: $range) -> $extended {
                $extended {
                    x,
                    y,
                    t: x * y,
                    z: $range::one(),
                }
            }

            fn is_identity(&self) -> bool {
                (self.x == $range::zero()) & (self.y == self.z)
            }

            fn is_on_curve(self) -> bool {
                if self.z.is_zero() {
                    true
                } else {
                    let affine = $affine::from(self);
                    affine.is_on_curve()
                }
            }

            fn get_x(&self) -> Self::Base {
                self.x
            }

            fn get_y(&self) -> Self::Base {
                self.y
            }
        }

        impl TwistedEdwardsCurve for $extended {
            const PARAM_D: $range = $d;
            type Scalar = $scalar;
        }

        impl AddAssign for $extended {
            fn add_assign(&mut self, rhs: $extended) {
                *self = *self + rhs;
            }
        }

        impl<'b> AddAssign<&'b $extended> for $extended {
            fn add_assign(&mut self, rhs: &'b $extended) {
                *self += *rhs;
            }
        }

        impl SubAssign for $extended {
            fn sub_assign(&mut self, rhs: $extended) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $extended> for $extended {
            fn sub_assign(&mut self, rhs: &'b $extended) {
                *self -= *rhs;
            }
        }

        impl MulAssign<$scalar> for $extended {
            fn mul_assign(&mut self, rhs: $scalar) {
                *self = *self * rhs;
            }
        }

        impl<'b> MulAssign<&'b $scalar> for $extended {
            fn mul_assign(&mut self, rhs: &'b $scalar) {
                *self *= *rhs;
            }
        }
    };
}

pub use {twisted_edwards_affine_group_operation, twisted_edwards_extend_group_operation};
