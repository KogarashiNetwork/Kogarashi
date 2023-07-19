pub mod edwards;
pub mod weierstrass;

#[macro_export]
macro_rules! curve_arithmetic_extension {
    ($curve:ident, $scalar:ident, $extended:ident) => {
        impl Eq for $curve {}

        impl Default for $curve {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl<'a, 'b> Add<&'b $curve> for &'a $curve {
            type Output = $extended;

            fn add(self, rhs: &'b $curve) -> $extended {
                *self + *rhs
            }
        }

        impl<'b> Add<&'b $curve> for $curve {
            type Output = $extended;

            fn add(self, rhs: &'b $curve) -> $extended {
                &self + rhs
            }
        }

        impl<'a> Add<$curve> for &'a $curve {
            type Output = $extended;

            fn add(self, rhs: $curve) -> $extended {
                self + &rhs
            }
        }

        impl<'a, 'b> Sub<&'b $curve> for &'a $curve {
            type Output = $extended;

            fn sub(self, rhs: &'b $curve) -> $extended {
                *self - *rhs
            }
        }

        impl<'b> Sub<&'b $curve> for $curve {
            type Output = $extended;

            fn sub(self, rhs: &'b $curve) -> $extended {
                &self - rhs
            }
        }

        impl<'a> Sub<$curve> for &'a $curve {
            type Output = $extended;

            fn sub(self, rhs: $curve) -> $extended {
                self - &rhs
            }
        }

        impl<'a> Mul<&'a $scalar> for $curve {
            type Output = $extended;

            fn mul(self, rhs: &'a $scalar) -> Self::Output {
                self * *rhs
            }
        }

        impl<'a> Mul<$scalar> for &'a $curve {
            type Output = $extended;

            fn mul(self, rhs: $scalar) -> Self::Output {
                *self * rhs
            }
        }

        impl<'a, 'b> Mul<&'b $scalar> for &'a $curve {
            type Output = $extended;

            fn mul(self, rhs: &'b $scalar) -> Self::Output {
                *self * *rhs
            }
        }

        impl<'a> Mul<&'a $curve> for $scalar {
            type Output = $extended;

            fn mul(self, rhs: &'a $curve) -> Self::Output {
                self * *rhs
            }
        }

        impl<'a> Mul<$curve> for &'a $scalar {
            type Output = $extended;

            fn mul(self, rhs: $curve) -> Self::Output {
                *self * rhs
            }
        }

        impl<'a, 'b> Mul<&'b $curve> for &'a $scalar {
            type Output = $extended;

            fn mul(self, rhs: &'b $curve) -> Self::Output {
                *self * *rhs
            }
        }
    };
}

#[macro_export]
macro_rules! mixed_curve_operations {
    ($affine:ident, $extended:ident) => {
        impl Add<$extended> for $affine {
            type Output = $extended;

            fn add(self, rhs: $extended) -> $extended {
                add_point(self.to_extended(), rhs)
            }
        }

        impl<'a, 'b> Add<&'b $extended> for &'a $affine {
            type Output = $extended;

            fn add(self, rhs: &'b $extended) -> $extended {
                *self + *rhs
            }
        }

        impl<'b> Add<&'b $extended> for $affine {
            type Output = $extended;

            fn add(self, rhs: &'b $extended) -> $extended {
                &self + rhs
            }
        }

        impl<'a> Add<$extended> for &'a $affine {
            type Output = $extended;

            fn add(self, rhs: $extended) -> $extended {
                self + &rhs
            }
        }

        impl Sub<$extended> for $affine {
            type Output = $extended;

            fn sub(self, rhs: $extended) -> $extended {
                add_point(self.to_extended(), -rhs)
            }
        }

        impl<'a, 'b> Sub<&'b $extended> for &'a $affine {
            type Output = $extended;

            fn sub(self, rhs: &'b $extended) -> $extended {
                *self + *rhs
            }
        }

        impl<'b> Sub<&'b $extended> for $affine {
            type Output = $extended;

            fn sub(self, rhs: &'b $extended) -> $extended {
                &self + rhs
            }
        }

        impl<'a> Sub<$extended> for &'a $affine {
            type Output = $extended;

            fn sub(self, rhs: $extended) -> $extended {
                self + &rhs
            }
        }

        impl Add<$affine> for $extended {
            type Output = $extended;

            fn add(self, rhs: $affine) -> $extended {
                add_point(self, rhs.to_extended())
            }
        }

        impl<'a, 'b> Add<&'b $affine> for &'a $extended {
            type Output = $extended;

            fn add(self, rhs: &'b $affine) -> $extended {
                *self + *rhs
            }
        }

        impl<'b> Add<&'b $affine> for $extended {
            type Output = $extended;

            fn add(self, rhs: &'b $affine) -> $extended {
                &self + rhs
            }
        }

        impl<'a> Add<$affine> for &'a $extended {
            type Output = $extended;

            fn add(self, rhs: $affine) -> $extended {
                self + &rhs
            }
        }

        impl Sub<$affine> for $extended {
            type Output = $extended;

            fn sub(self, rhs: $affine) -> $extended {
                add_point(self, -rhs.to_extended())
            }
        }

        impl<'a, 'b> Sub<&'b $affine> for &'a $extended {
            type Output = $extended;

            fn sub(self, rhs: &'b $affine) -> $extended {
                *self - *rhs
            }
        }

        impl<'b> Sub<&'b $affine> for $extended {
            type Output = $extended;

            fn sub(self, rhs: &'b $affine) -> $extended {
                &self - rhs
            }
        }

        impl<'a> Sub<$affine> for &'a $extended {
            type Output = $extended;

            fn sub(self, rhs: $affine) -> $extended {
                self - &rhs
            }
        }

        impl AddAssign<$affine> for $extended {
            fn add_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, rhs.to_extended())
            }
        }

        impl<'a> AddAssign<&'a $affine> for $extended {
            fn add_assign(&mut self, rhs: &'a $affine) {
                *self = add_point(*self, rhs.to_extended())
            }
        }

        impl SubAssign<$affine> for $extended {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, -rhs.to_extended())
            }
        }

        impl<'a> SubAssign<&'a $affine> for $extended {
            fn sub_assign(&mut self, rhs: &'a $affine) {
                *self = add_point(*self, -rhs.to_extended())
            }
        }
    };
}

pub use {curve_arithmetic_extension, mixed_curve_operations};
