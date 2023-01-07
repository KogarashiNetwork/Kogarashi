#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_arithmetic_extension!($affine, $scalar);

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

            fn add(self, rhs: $affine) -> Self {
                Self::from(add_point(self.to_projective(), rhs.to_projective()))
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
            type Output = Self;

            fn sub(self, rhs: $affine) -> Self {
                Self::from(add_point(self.to_projective(), rhs.neg().to_projective()))
            }
        }

        impl Mul<<Self as Group>::Scalar> for $affine {
            type Output = Self;

            fn mul(self, rhs: <Self as Group>::Scalar) -> Self {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = rhs.to_bits().into_iter().skip_while(|x| *x == 0).collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc = acc.double();
                }
                res
            }
        }

        impl<'b> Mul<&'b <Self as Group>::Scalar> for $affine {
            type Output = $affine;

            fn mul(self, rhs: &'b <Self as Group>::Scalar) -> $affine {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = rhs.to_bits().into_iter().skip_while(|x| *x == 0).collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc = acc.double();
                }
                res
            }
        }
    };
}

#[macro_export]
macro_rules! projective_group_operation {
    ($projective:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        curve_arithmetic_extension!($projective, $scalar);

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

        impl Mul<<Self as Group>::Scalar> for $projective {
            type Output = Self;

            fn mul(self, scalar: <Self as Group>::Scalar) -> Self {
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

        impl<'b> Mul<&'b <Self as Group>::Scalar> for $projective {
            type Output = $projective;

            fn mul(self, rhs: &'b <Self as Group>::Scalar) -> $projective {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = rhs.to_bits().into_iter().skip_while(|x| *x == 0).collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc = acc.double();
                }
                res
            }
        }
    };
}

#[macro_export]
macro_rules! curve_arithmetic_extension {
    ($curve:ident, $scalar:ident) => {
        impl Eq for $curve {}

        impl Default for $curve {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl AddAssign for $curve {
            fn add_assign(&mut self, rhs: $curve) {
                *self = *self + rhs;
            }
        }

        impl<'b> AddAssign<&'b $curve> for $curve {
            fn add_assign(&mut self, rhs: &'b $curve) {
                *self = &*self + rhs;
            }
        }

        impl<'a, 'b> Add<&'b $curve> for &'a $curve {
            type Output = $curve;

            fn add(self, rhs: &'b $curve) -> $curve {
                self + rhs
            }
        }

        impl<'b> Add<&'b $curve> for $curve {
            type Output = $curve;

            fn add(self, rhs: &'b $curve) -> Self {
                &self + rhs
            }
        }

        impl<'a> Add<$curve> for &'a $curve {
            type Output = $curve;

            fn add(self, rhs: $curve) -> $curve {
                self + rhs
            }
        }

        impl SubAssign for $curve {
            fn sub_assign(&mut self, rhs: $curve) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $curve> for $curve {
            fn sub_assign(&mut self, rhs: &'b $curve) {
                *self = &*self - rhs;
            }
        }

        impl<'a, 'b> Sub<&'b $curve> for &'a $curve {
            type Output = $curve;

            fn sub(self, rhs: &'b $curve) -> $curve {
                self - rhs
            }
        }

        impl<'b> Sub<&'b $curve> for $curve {
            type Output = $curve;

            fn sub(self, rhs: &'b $curve) -> Self {
                self - rhs
            }
        }

        impl<'a> Sub<$curve> for &'a $curve {
            type Output = $curve;

            fn sub(self, rhs: $curve) -> $curve {
                self - rhs
            }
        }

        impl MulAssign<<Self as Group>::Scalar> for $curve {
            fn mul_assign(&mut self, rhs: <Self as Group>::Scalar) {
                *self = *self * rhs;
            }
        }
    };
}

pub use {affine_group_operation, curve_arithmetic_extension, projective_group_operation};
