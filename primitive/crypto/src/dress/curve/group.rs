#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_repr_common_operation!($affine, $scalar);

        impl Group for $affine {
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

            fn zero() -> Self {
                Self::ADDITIVE_GENERATOR
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

            fn random(rand: impl RngCore) -> Self {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x == other.x && self.y == other.y
                }
            }
        }

        impl Add for $affine {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $affine) -> Self {
                Self::from(add_point(self.to_projective(), rhs.to_projective()))
            }
        }

        impl Neg for $affine {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    is_infinity: self.is_infinity,
                }
            }
        }

        impl Sub for $affine {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $affine) -> Self {
                Self::from(add_point(self.to_projective(), rhs.neg().to_projective()))
            }
        }
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_repr_common_operation!($projective, $scalar);

        impl Group for $projective {
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

        impl PartialEq for $projective {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
                }
            }
        }

        impl Add for $projective {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $projective) -> Self {
                add_point(self, rhs)
            }
        }

        impl Neg for $projective {
            type Output = Self;

            #[inline]
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

            #[inline]
            fn sub(self, rhs: $projective) -> Self {
                add_point(self, -rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! curve_repr_common_operation {
    ($curve_repr:ident, $scalar:ident) => {
        impl Eq for $curve_repr {}

        impl Default for $curve_repr {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl AddAssign for $curve_repr {
            fn add_assign(&mut self, rhs: $curve_repr) {
                *self = *self + rhs;
            }
        }

        impl SubAssign for $curve_repr {
            fn sub_assign(&mut self, rhs: $curve_repr) {
                *self = *self - rhs;
            }
        }

        impl Mul<$scalar> for $curve_repr {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $scalar) -> Self {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .to_bits()
                    .into_iter()
                    .skip_while(|x| *x == 0)
                    .collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc = acc.double();
                }
                res
            }
        }

        impl MulAssign<$scalar> for $curve_repr {
            fn mul_assign(&mut self, scalar: $scalar) {
                *self = *self * scalar;
            }
        }
    };
}

pub use {affine_group_operation, curve_repr_common_operation, projective_group_operation};
