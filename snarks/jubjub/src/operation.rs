macro_rules! field_operation {
    ($field:ident) => {
        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                $field(add(&self.0, &rhs.0))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 = add(&self.0, &rhs.0)
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(&self.0, &rhs.0))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 = sub(&self.0, &rhs.0)
            }
        }

        impl Mul for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: $field) -> $field {
                $field(mul(&self.0, &rhs.0))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(&self.0, &rhs.0))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 = mul(&self.0, &rhs.0)
            }
        }

        impl Neg for $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
                -&self
            }
        }

        impl<'a> Neg for &'a $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
                $field(neg(&self.0))
            }
        }
    };
}

pub(crate) use field_operation;
