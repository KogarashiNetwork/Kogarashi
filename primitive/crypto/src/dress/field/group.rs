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
                self.0[0] == other.0[0]
                    && self.0[1] == other.0[1]
                    && self.0[2] == other.0[2]
                    && self.0[3] == other.0[3]
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

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                $field(add(self.0, rhs.0, $p))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 = add(self.0, rhs.0, $p)
            }
        }

        impl Neg for $field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $field(neg(self.0, $p))
            }
        }

        impl<'a> Neg for &'a $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
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

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(self.0, rhs.0, $p))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 = sub(self.0, rhs.0, $p)
            }
        }

        impl Mul<<Self as Group>::Scalar> for $field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $field) -> Self {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl MulAssign<<Self as Group>::Scalar> for $field {
            fn mul_assign(&mut self, rhs: $field) {
                *self = $field(mul(self.0, rhs.0, $p, $inv))
            }
        }
    };
}

pub use group_operation;
