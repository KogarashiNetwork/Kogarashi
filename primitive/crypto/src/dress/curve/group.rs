#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        impl Group for $affine {
            type Scalar = $scalar;

            const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                is_infinity: false,
            };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::zero(),
                is_infinity: false,
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

        impl Eq for $affine {}

        impl Add for $affine {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $affine) -> Self {
                Self::from(add_point(self.to_projective(), rhs.to_projective()))
            }
        }

        impl AddAssign for $affine {
            fn add_assign(&mut self, rhs: $affine) {
                *self = *self + rhs;
            }
        }

        impl<'a, 'b> Mul<&'b $scalar> for &'a $affine {
            type Output = $affine;

            #[inline]
            fn mul(self, scalar: &'b $scalar) -> $affine {
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
                    acc.double();
                }
                res
            }
        }

        impl MulAssign<$scalar> for $affine {
            fn mul_assign(&mut self, scalar: $scalar) {
                *self = *self * scalar;
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

        impl<'a> Neg for &'a $affine {
            type Output = $affine;

            #[inline]
            fn neg(self) -> $affine {
                $affine {
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

        impl SubAssign for $affine {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = self.add(rhs.neg());
            }
        }
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
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

        impl Eq for $projective {}

        impl Add for $projective {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $projective) -> Self {
                add_point(self, rhs)
            }
        }

        impl<'a, 'b> Add<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn add(self, rhs: &'b $projective) -> $projective {
                add_point(self.clone(), rhs.clone())
            }
        }

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                *self = self.add(rhs);
            }
        }

        impl<'a, 'b> Mul<&'b $scalar> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn mul(self, scalar: &'b $scalar) -> $projective {
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
                    acc.double();
                }
                res
            }
        }

        impl MulAssign<$scalar> for $projective {
            fn mul_assign(&mut self, scalar: $scalar) {
                *self = *self * scalar;
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

        impl<'a> Neg for &'a $projective {
            type Output = $projective;

            #[inline]
            fn neg(self) -> $projective {
                $projective {
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
                add_point(self, rhs.neg())
            }
        }

        impl<'a, 'b> Sub<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn sub(self, rhs: &'b $projective) -> $projective {
                add_point(self.clone(), rhs.neg())
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                *self = self.add(rhs.neg());
            }
        }
    };
}

pub use {affine_group_operation, projective_group_operation};
