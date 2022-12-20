#[macro_export]
macro_rules! ring_operation {
    ($field:ident, $p:ident, $g:ident, $r:ident, $inv:ident) => {
        group_operation!($field, $p, $g, $r, $inv);

        impl Ring for $field {}

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
    };
}

pub use ring_operation;
