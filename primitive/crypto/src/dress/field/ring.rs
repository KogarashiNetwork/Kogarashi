#[macro_export]
macro_rules! ring_operation {
    ($field:ident, $p:ident) => {
        impl Ring for $field {}

        impl Add for $field {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $field) -> Self {
                $field(add(&self.0, &rhs.0, &$p.0))
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a Self {
            type Output = Self;

            #[inline]
            fn add(self, rhs: &'b $field) -> Self {
                $field(add(&self.0, &rhs.0, &$p.0))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 = add(&self.0, &rhs.0, &$p.0)
            }
        }

        impl Mul for $field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $field) -> Self {
                $field(mul(&self.0, &rhs.0, &$p.0))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a Self {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: &'b $field) -> Self {
                $field(mul(&self.0, &rhs.0, &$p.0))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 = mul(&self.0, &rhs.0, &$p.0)
            }
        }

        impl Neg for $field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                -&self
            }
        }

        impl<'a> Neg for &'a $field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $field(neg(&self.0, &$p.0))
            }
        }

        impl Sub for $field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $field) -> Self {
                $field(sub(&self.0, &rhs.0, &$p.0))
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: &'b $field) -> Self {
                $field(sub(&self.0, &rhs.0, &$p.0))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 = sub(&self.0, &rhs.0, &$p.0)
            }
        }
    };
}

pub use ring_operation;
