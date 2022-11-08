#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident) => {
        // basic arithmetic
        impl $field {
            #[inline(always)]
            pub fn double_assign(&mut self) {
                self.0 = double(&self.0, $p)
            }

            #[inline(always)]
            pub fn square_assign(&mut self) {
                self.0 = square(&self.0, $p)
            }

            pub const fn zero() -> $field {
                $field([0, 0, 0, 0])
            }
        }

        impl Add for $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: $field) -> $field {
                $field(add(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                $field(add(&self.0, &rhs.0, $p))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 = add(&self.0, &rhs.0, $p)
            }
        }

        impl Sub for $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: $field) -> $field {
                $field(sub(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(&self.0, &rhs.0, $p))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 = sub(&self.0, &rhs.0, $p)
            }
        }

        impl Mul for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: $field) -> $field {
                $field(mul(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(&self.0, &rhs.0, $p))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 = mul(&self.0, &rhs.0, $p)
            }
        }

        impl $field {
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|x| *x == 0)
            }

            pub fn double(self) -> $field {
                $field(double(&self.0, $p))
            }

            pub fn square(self) -> $field {
                $field(square(&self.0, $p))
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
                $field(neg(&self.0, $p))
            }
        }

        // comparison operation
        impl Eq for Fr {}

        impl PartialEq for Fr {
            fn eq(&self, other: &Self) -> bool {
                self.0[0] == other.0[0]
                    && self.0[1] == other.0[1]
                    && self.0[2] == other.0[2]
                    && self.0[3] == other.0[3]
            }
        }

        impl PartialOrd for $field {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }

            fn lt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                false
            }

            fn le(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                true
            }

            fn gt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                false
            }

            fn ge(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                true
            }
        }

        impl Ord for Fr {
            fn cmp(&self, other: &Self) -> Ordering {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a < b {
                        return Ordering::Less;
                    } else if a > b {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
        }

        // basic trait
        impl Default for $field {
            fn default() -> Self {
                $field::zero()
            }
        }

        impl Display for $field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use field_operation;
