#[macro_export]
macro_rules! group_operation {
    ($field:ident, $p:ident, $g:ident, $r:ident, $r2:ident, $r3:ident, $inv:ident) => {
        group_arithmetic_extension!($field);

        impl Group for $field {
            type Scalar = $field;

            const ADDITIVE_GENERATOR: Self = $field($g);
            const ADDITIVE_IDENTITY: Self = $field($r);

            fn zero() -> Self {
                Self(zero())
            }

            fn invert(self) -> Option<Self> {
                match invert(self.0, little_fermat($p), $r, $p, $inv) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
            }

            fn random(rand: impl RngCore) -> Self {
                Self(random_limbs(rand, $r2, $r3, $p, $inv))
            }
        }

        impl PartialEq for $field {
            fn eq(&self, other: &Self) -> bool {
                self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
            }
        }

        impl Eq for $field {}

        impl Add for $field {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $field) -> Self {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl Neg for $field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $field(neg(self.0, $p))
            }
        }

        impl Sub for $field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $field) -> Self {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl Mul<<Self as Group>::Scalar> for $field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $field) -> Self {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl $field {
            pub const fn zero() -> Self {
                Self(zero())
            }

            pub const fn one() -> Self {
                Self($r)
            }
        }
    };
}

#[macro_export]
macro_rules! group_arithmetic_extension {
    ($field:ident) => {
        impl<'a> Neg for &'a $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
                -self
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                *self = *self + rhs;
            }
        }

        impl<'b> AddAssign<&'b $field> for $field {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $field) {
                *self = &*self + rhs;
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                self + rhs
            }
        }

        impl<'b> Add<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> Self {
                &self + rhs
            }
        }

        impl<'a> Add<$field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: $field) -> $field {
                self + rhs
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $field> for $field {
            #[inline]
            fn sub_assign(&mut self, rhs: &'b $field) {
                *self = &*self - rhs;
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                self - rhs
            }
        }

        impl<'b> Sub<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> Self {
                self - rhs
            }
        }

        impl<'a> Sub<$field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: $field) -> $field {
                self - rhs
            }
        }

        impl MulAssign<<Self as Group>::Scalar> for $field {
            fn mul_assign(&mut self, rhs: <Self as Group>::Scalar) {
                *self = *self * rhs;
            }
        }

        impl<'b> MulAssign<&'b $field> for $field {
            #[inline]
            fn mul_assign(&mut self, rhs: &'b $field) {
                *self = &*self * rhs;
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                self * rhs
            }
        }

        impl<'b> Mul<&'b $field> for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                self * rhs
            }
        }

        impl<'a> Mul<$field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: $field) -> $field {
                self * rhs
            }
        }
    };
}

pub use {group_arithmetic_extension, group_operation};
