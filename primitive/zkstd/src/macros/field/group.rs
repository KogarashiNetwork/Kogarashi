/// abstract algebra group operation macro
#[macro_export]
macro_rules! group_operation {
    ($field:ident, $p:ident, $g:ident, $r:ident, $r2:ident, $r3:ident, $inv:ident) => {
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

            fn add(self, rhs: $field) -> Self {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            fn add(self, rhs: &'b $field) -> $field {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl<'a> Add<$field> for &'a $field {
            type Output = $field;

            fn add(self, rhs: $field) -> $field {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl<'b> AddAssign<&'b $field> for $field {
            fn add_assign(&mut self, rhs: &'b $field) {
                *self = $field(add(self.0, rhs.0, $p))
            }
        }

        impl<'b> Add<&'b $field> for $field {
            type Output = $field;

            fn add(self, rhs: &'b $field) -> Self {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl Neg for $field {
            type Output = Self;

            fn neg(self) -> Self {
                $field(neg(self.0, $p))
            }
        }

        impl Sub for $field {
            type Output = Self;

            fn sub(self, rhs: $field) -> Self {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl Mul<$field> for $field {
            type Output = Self;

            fn mul(self, rhs: $field) -> Self {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'b> MulAssign<&'b $field> for $field {
            fn mul_assign(&mut self, rhs: &'b $field) {
                *self = &*self * rhs;
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'b> Mul<&'b $field> for $field {
            type Output = $field;

            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'a> Mul<$field> for &'a $field {
            type Output = $field;

            fn mul(self, rhs: $field) -> $field {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'a> Neg for &'a $field {
            type Output = $field;

            fn neg(self) -> $field {
                -self
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                *self = *self + rhs;
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                *self = *self - rhs;
            }
        }

        impl<'b> SubAssign<&'b $field> for $field {
            fn sub_assign(&mut self, rhs: &'b $field) {
                *self = $field(sub(self.0, rhs.0, $p))
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl<'b> Sub<&'b $field> for $field {
            type Output = $field;

            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl<'a> Sub<$field> for &'a $field {
            type Output = $field;

            fn sub(self, rhs: $field) -> $field {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl MulAssign<$field> for $field {
            fn mul_assign(&mut self, rhs: $field) {
                *self = *self * rhs;
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

pub use group_operation;
