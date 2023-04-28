#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
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

        impl CurveGroup for $affine {
            type Affine = $affine;
            type Extended = $projective;
            type Scalar = $scalar;

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

            fn is_identity(self) -> bool {
                self.is_infinity
            }

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

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

            fn random(rand: impl RngCore) -> $projective {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl Add for $affine {
            type Output = $projective;

            fn add(self, rhs: $affine) -> Self::Output {
                $projective::from(add_point(self.to_extended(), rhs.to_extended()))
            }
        }

        impl Neg for $affine {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    is_infinity: self.is_infinity,
                }
            }
        }

        impl Sub for $affine {
            type Output = $projective;

            fn sub(self, rhs: $affine) -> Self::Output {
                $projective::from(add_point(self.to_extended(), rhs.neg().to_extended()))
            }
        }

        impl Mul<$scalar> for $affine {
            type Output = $projective;

            fn mul(self, rhs: $scalar) -> Self::Output {
                scalar_point(self.to_extended(), &rhs)
            }
        }

        impl Mul<$affine> for $scalar {
            type Output = $projective;

            fn mul(self, rhs: $affine) -> Self::Output {
                scalar_point(rhs.to_extended(), &self)
            }
        }
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($affine: ident, $projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
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

        impl CurveGroup for $projective {
            type Affine = $affine;
            type Extended = $projective;
            type Scalar = $scalar;

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

            fn is_identity(self) -> bool {
                self.z == $range::zero()
            }

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

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

        impl Add for $projective {
            type Output = Self;

            fn add(self, rhs: $projective) -> Self {
                add_point(self, rhs)
            }
        }

        impl Neg for $projective {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    z: self.z,
                }
            }
        }

        impl Sub for $projective {
            type Output = Self;

            fn sub(self, rhs: $projective) -> Self {
                add_point(self, -rhs)
            }
        }

        impl Mul<$scalar> for $projective {
            type Output = $projective;

            fn mul(self, rhs: $scalar) -> Self::Output {
                scalar_point(self, &rhs)
            }
        }

        impl Mul<$projective> for $scalar {
            type Output = $projective;

            fn mul(self, rhs: $projective) -> Self::Output {
                scalar_point(rhs, &self)
            }
        }

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                *self = (*self + rhs).into();
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
