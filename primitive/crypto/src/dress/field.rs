mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident) => {
        ring_operation!($field, $p, $g, $e, $inv);

        impl Field for $field {}

        impl Mul for $field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $field) -> Self {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(self.0, rhs.0, $p, $inv))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 = mul(self.0, rhs.0, $p, $inv)
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Div for $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: $field) -> $field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<'a, 'b> Div<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: &'b $field) -> $field {
                let inv = rhs.invert().unwrap();
                self * &inv
            }
        }

        #[allow(clippy::suspicious_op_assign_impl)]
        impl DivAssign for $field {
            fn div_assign(&mut self, rhs: $field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }
    };
}

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        field_operation!($field, $p, $g, $r, $inv);

        field_built_in!($field);

        const_field_operation!($field, $r, $r2, $p, $inv);

        impl PrimeField for $field {
            const MODULUS: Self = $field($p);

            const INV: u64 = $inv;

            fn from_u64(val: u64) -> Self {
                Self(from_u64(val))
            }

            fn to_bits(self) -> Bits {
                to_bits(self.0)
            }

            fn is_zero(self) -> bool {
                self.0.iter().all(|x| *x == 0)
            }

            fn random(rand: impl RngCore) -> Self {
                Self(random_limbs(rand, $r2, $r3, $p, $inv))
            }

            fn double(self) -> Self {
                Self(double(self.0, $p))
            }

            fn square(self) -> Self {
                Self(square(self.0, $p, $inv))
            }

            fn double_assign(&mut self) {
                self.0 = double(self.0, $p)
            }

            fn square_assign(&mut self) {
                self.0 = square(self.0, $p, $inv)
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

        impl Ord for $field {
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
    };
}

#[macro_export]
macro_rules! fft_field_operation {
    ($field:ident, $p:ident, $g:ident, $i:ident, $u:ident, $r:ident, $r2:ident, $r3:ident, $s:ident) => {
        prime_field_operation!($field, $p, $g, $i, $r, $r2, $r3);

        impl FftField for $field {
            const S: usize = $s;

            const ROOT_OF_UNITY: Self = $u;

            fn zero() -> Self {
                $field(zero())
            }

            fn one() -> Self {
                $field(one($r2, $p, $i))
            }
        }

        impl From<u64> for $field {
            fn from(val: u64) -> $field {
                $field(mul(from_u64(val), $r2, $p, $i))
            }
        }

        impl ParallelCmp for $field {}
    };
}

#[macro_export]
macro_rules! pairing_field_operation {
    ($field:ident, $p:ident, $g:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        prime_field_operation!($field, $p, $g, $inv, $r, $r2, $r3);

        impl PairingField for $field {}
    };
}

#[macro_export]
macro_rules! field_built_in {
    ($element:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $element {}

        impl Basic for $element {}

        impl Default for $element {
            fn default() -> Self {
                Self(zero())
            }
        }

        impl Display for $element {
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

#[macro_export]
macro_rules! const_field_operation {
    ($field:ident, $r:ident, $r2:ident, $p:ident, $inv:ident) => {
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

pub use {
    const_field_operation, fft_field_operation, field_built_in, field_operation,
    pairing_field_operation, prime_field_operation,
};
