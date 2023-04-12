#[macro_export]
macro_rules! twisted_edwards_affine_group_operation {
    ($affine:ident, $extend:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        twisted_edwards_curve_arithmetic_extension!($affine, $scalar);
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl CurveGroup for $affine {
            type Affine = $affine;
            type Projective = $extend;
            type Scalar = $scalar;

            const ADDITIVE_GENERATOR: Self = Self { x: $x, y: $y };

            const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
            };

            fn is_identity(self) -> bool {
                self == Self::ADDITIVE_IDENTITY
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

            fn random(rand: impl RngCore) -> $extend {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl Add for $affine {
            type Output = $extend;

            fn add(self, rhs: $affine) -> Self::Output {
                $extend::from(add_point(self.to_extend(), rhs.to_extend()))
            }
        }

        impl Neg for $affine {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: self.y,
                }
            }
        }

        impl Sub for $affine {
            type Output = $extend;

            fn sub(self, rhs: $affine) -> Self::Output {
                $extend::from(add_point(self.to_extend(), rhs.neg().to_extend()))
            }
        }

        impl Mul<<Self as CurveGroup>::Scalar> for $affine {
            type Output = $extend;

            fn mul(self, rhs: <Self as CurveGroup>::Scalar) -> Self::Output {
                scalar_point(self.to_extend(), &rhs)
            }
        }

        impl<'b> Mul<&'b <Self as CurveGroup>::Scalar> for $affine {
            type Output = $extend;

            fn mul(self, rhs: &'b <Self as CurveGroup>::Scalar) -> Self::Output {
                scalar_point(self.to_extend(), rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! twisted_edwards_extend_group_operation {
    ($affine:ident, $extend:ident, $range:ident, $scalar:ident, $x:ident, $y:ident, $t:ident) => {
        twisted_edwards_curve_arithmetic_extension!($extend, $scalar);

        impl CurveGroup for $extend {
            type Affine = $affine;
            type Projective = $extend;
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

            fn is_identity(self) -> bool {
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

        impl PartialEq for $extend {
            fn eq(&self, other: &Self) -> bool {
                self.x * other.z == other.x * self.z && self.y * &other.z == other.y * self.z
            }
        }

        impl Add for $extend {
            type Output = $extend;

            fn add(self, rhs: $extend) -> Self::Output {
                $extend::from(add_point(self, rhs))
            }
        }

        impl Neg for $extend {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: self.y,
                    t: -self.t,
                    z: self.z,
                }
            }
        }

        impl Sub for $extend {
            type Output = $extend;

            fn sub(self, rhs: $extend) -> Self::Output {
                $extend::from(add_point(self, rhs.neg()))
            }
        }

        impl Mul<<Self as CurveGroup>::Scalar> for $extend {
            type Output = $extend;

            fn mul(self, rhs: <Self as CurveGroup>::Scalar) -> Self::Output {
                scalar_point(self, &rhs)
            }
        }

        impl<'b> Mul<&'b <Self as CurveGroup>::Scalar> for $extend {
            type Output = $extend;

            fn mul(self, rhs: &'b <Self as CurveGroup>::Scalar) -> Self::Output {
                scalar_point(self, rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! twisted_edwards_curve_arithmetic_extension {
    ($curve:ident, $scalar:ident) => {
        impl Eq for $curve {}

        impl Default for $curve {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl AddAssign for $curve {
            fn add_assign(&mut self, rhs: $curve) {
                *self = (*self + rhs).into();
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
                *self = (*self - rhs).into();
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

        impl MulAssign<<Self as CurveGroup>::Scalar> for $curve {
            fn mul_assign(&mut self, rhs: <Self as CurveGroup>::Scalar) {
                *self = (*self * rhs).into();
            }
        }
    };
}

pub use {
    twisted_edwards_affine_group_operation, twisted_edwards_curve_arithmetic_extension,
    twisted_edwards_extend_group_operation,
};
