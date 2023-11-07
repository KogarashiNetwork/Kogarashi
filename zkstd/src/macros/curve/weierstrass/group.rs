#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $a:ident, $b:ident, $b3:ident) => {
        curve_arithmetic_extension!($affine, $scalar, $projective);
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x == other.x && self.y == other.y
                }
            }
        }

        impl Group for $affine {
            const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                is_infinity: false,
            };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
                is_infinity: true,
            };

            fn invert(self) -> Option<Self> {
                match self.is_infinity {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                        is_infinity: false,
                    }),
                }
            }

            fn random(rand: impl RngCore) -> $affine {
                $affine::from(Self::ADDITIVE_GENERATOR * $scalar::random(rand))
            }
        }

        impl WeierstrassCurve for $affine {
            const PARAM_B: $range = $b;
            const PARAM_3B: $range = $b3;

            type Range = $range;
            type Scalar = $scalar;

            fn is_identity(&self) -> bool {
                self.is_infinity
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
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($affine: ident, $projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $a:ident, $b:ident, $b3:ident) => {
        curve_arithmetic_extension!($projective, $scalar, $projective);

        impl PartialEq for $projective {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
                }
            }
        }

        impl Group for $projective {
            const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                z: $range::one(),
            };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
                z: $range::zero(),
            };

            fn invert(self) -> Option<Self> {
                match self.z.is_zero() {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                        z: self.z,
                    }),
                }
            }

            fn random(rand: impl RngCore) -> Self {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl WeierstrassCurve for $projective {
            const PARAM_B: $range = $b;
            const PARAM_3B: $range = $b3;
            type Range = $range;
            type Scalar = $scalar;

            fn is_identity(&self) -> bool {
                self.z == $range::zero()
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

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                *self = *self + rhs;
            }
        }

        impl<'a> AddAssign<$projective> for &'a $projective {
            fn add_assign(&mut self, rhs: $projective) {
                *self += rhs;
            }
        }

        impl<'b> AddAssign<&'b $projective> for $projective {
            fn add_assign(&mut self, rhs: &'b $projective) {
                *self += *rhs;
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $projective> for $projective {
            fn sub_assign(&mut self, rhs: &'b $projective) {
                *self -= *rhs;
            }
        }

        impl MulAssign<$scalar> for $projective {
            fn mul_assign(&mut self, rhs: $scalar) {
                *self = *self * rhs;
            }
        }

        impl<'b> MulAssign<&'b $scalar> for $projective {
            fn mul_assign(&mut self, rhs: &'b $scalar) {
                *self *= *rhs;
            }
        }
    };
}

pub use {affine_group_operation, projective_group_operation};
