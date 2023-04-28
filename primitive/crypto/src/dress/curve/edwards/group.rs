#[macro_export]
macro_rules! twisted_edwards_affine_group_operation {
    ($affine:ident, $extended:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_arithmetic_extension!($affine, $scalar, $extended);
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl CurveGroup for $affine {
            type Affine = $affine;
            type Extended = $extended;
            type Scalar = $scalar;

            const ADDITIVE_GENERATOR: Self = Self { x: $x, y: $y };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
            };

            fn is_identity(&self) -> bool {
                self == &Self::ADDITIVE_IDENTITY
            }

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

            fn invert(self) -> Option<Self> {
                match self.x.is_zero() {
                    true => None,
                    false => Some(Self {
                        x: -self.x,
                        y: self.y,
                    }),
                }
            }

            fn random(rand: impl RngCore) -> $extended {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }
    };
}

#[macro_export]
macro_rules! twisted_edwards_extend_group_operation {
    ($affine:ident, $extended:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $t:ident) => {
        curve_arithmetic_extension!($extended, $scalar, $extended);

        impl PartialEq for $extended {
            fn eq(&self, other: &Self) -> bool {
                self.x * other.z == other.x * self.z && self.y * &other.z == other.y * self.z
            }
        }

        impl CurveGroup for $extended {
            type Affine = $affine;
            type Extended = $extended;
            type Scalar = $scalar;

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

            fn is_identity(&self) -> bool {
                self.x == $range::zero() && self.y == $range::one()
            }

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

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

            fn random(rand: impl RngCore) -> Self {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
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
