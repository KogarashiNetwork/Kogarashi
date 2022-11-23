#[macro_export]
macro_rules! extention_field_ring_operation {
    ($extention_field:ident) => {
        group_operation!($extention_field, $g);

        impl Ring for $extention_field {}

        impl Add for $extention_field {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $extention_field) -> Self {
                $extention_field([self.0[0] + rhs.0[1]])
            }
        }

        impl<'a, 'b> Add<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn add(self, rhs: &'b $extention_field) -> $extention_field {
                $extention_field(add(self.0, rhs.0, $p))
            }
        }

        impl AddAssign for $extention_field {
            fn add_assign(&mut self, rhs: $extention_field) {
                self.0 = add(self.0, rhs.0, $p)
            }
        }

        impl Neg for $extention_field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $extention_field(neg(self.0, $p))
            }
        }

        impl<'a> Neg for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn neg(self) -> $extention_field {
                $extention_field(neg(self.0, $p))
            }
        }

        impl Sub for $extention_field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $extention_field) -> Self {
                $extention_field(sub(self.0, rhs.0, $p))
            }
        }

        impl<'a, 'b> Sub<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn sub(self, rhs: &'b $extention_field) -> $extention_field {
                $extention_field(sub(self.0, rhs.0, $p))
            }
        }

        impl SubAssign for $extention_field {
            fn sub_assign(&mut self, rhs: $extention_field) {
                self.0 = sub(self.0, rhs.0, $p)
            }
        }
    };
}

pub use extention_field_ring_operation;
