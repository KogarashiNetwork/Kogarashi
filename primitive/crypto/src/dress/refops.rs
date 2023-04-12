#[macro_export]
macro_rules! ref_ops {
    ($t:ident) => {
        impl<'a, 'b> Add<&'b $t> for &'a $t {
            type Output = $t;

            fn add(self, rhs: &'b $t) -> $t {
                $t(add(self.0, rhs.0, $p))
            }
        }

        impl<'a> Add<$t> for &'a $t {
            type Output = $t;

            fn add(self, rhs: $t) -> $t {
                $t(add(self.0, rhs.0, $p))
            }
        }

        impl<'b> AddAssign<&'b $t> for $t {
            fn add_assign(&mut self, rhs: &'b $t) {
                *self = $t(add(self.0, rhs.0, $p))
            }
        }

        impl<'b> Add<&'b $t> for $t {
            type Output = $t;

            fn add(self, rhs: &'b $t) -> Self {
                $t(add(self.0, rhs.0, $p))
            }
        }

        impl<'b> MulAssign<&'b $t> for $t {
            fn mul_assign(&mut self, rhs: &'b $t) {
                *self = &*self * rhs;
            }
        }

        impl<'a, 'b> Mul<&'b $t> for &'a $t {
            type Output = $t;

            fn mul(self, rhs: &'b $t) -> $t {
                $t(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'b> Mul<&'b $t> for $t {
            type Output = $t;

            fn mul(self, rhs: &'b $t) -> $t {
                $t(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'a> Mul<$t> for &'a $t {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                $t(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'a> Neg for &'a $t {
            type Output = $t;

            fn neg(self) -> $t {
                -self
            }
        }

        impl<'b> SubAssign<&'b $t> for $t {
            fn sub_assign(&mut self, rhs: &'b $t) {
                *self = $t(sub(self.0, rhs.0, $p))
            }
        }

        impl<'a, 'b> Sub<&'b $t> for &'a $t {
            type Output = $t;

            fn sub(self, rhs: &'b $t) -> $t {
                $t(sub(self.0, rhs.0, $p))
            }
        }

        impl<'b> Sub<&'b $t> for $t {
            type Output = $t;

            fn sub(self, rhs: &'b $t) -> $t {
                $t(sub(self.0, rhs.0, $p))
            }
        }

        impl<'a> Sub<$t> for &'a $t {
            type Output = $t;

            fn sub(self, rhs: $t) -> $t {
                $t(sub(self.0, rhs.0, $p))
            }
        }
    };
}

pub use ref_ops;
