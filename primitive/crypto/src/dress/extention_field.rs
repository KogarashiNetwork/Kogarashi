mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! extention_field_operation {
    ($extention_field:ident, $sub_field:ident, $g:ident) => {
        extention_field_ring_operation!($extention_field, $g);

        extention_field_built_in!($extention_field);

        const_extention_field_operation!($extention_field, $sub_field);

        impl ExtentionField for $extention_field {}

        impl Field for $extention_field {}

        impl Mul for $extention_field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $extention_field) -> Self {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                $extention_field([re, im])
            }
        }

        impl<'a, 'b> Mul<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn mul(self, rhs: &'b $extention_field) -> $extention_field {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                $extention_field([re, im])
            }
        }

        impl MulAssign for $extention_field {
            fn mul_assign(&mut self, rhs: $extention_field) {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                self.0 = [re, im]
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Div for $extention_field {
            type Output = $extention_field;

            #[inline]
            fn div(self, rhs: $extention_field) -> $extention_field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<'a, 'b> Div<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn div(self, rhs: &'b $extention_field) -> $extention_field {
                let inv = rhs.invert().unwrap();
                self * &inv
            }
        }

        #[allow(clippy::suspicious_op_assign_impl)]
        impl DivAssign for $extention_field {
            fn div_assign(&mut self, rhs: $extention_field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }

        impl PrimeField for $extention_field {
            // wrong if this is problem
            const MODULUS: $extention_field = $g;

            const INV: u64 = $sub_field::INV;

            fn from_u64(val: u64) -> Self {
                unimplemented!()
            }

            fn to_bits(self) -> Bits {
                unimplemented!()
            }

            fn is_zero(self) -> bool {
                self.0[0].is_zero() & self.0[1].is_zero()
            }

            fn random(mut rand: impl RngCore) -> Self {
                $extention_field([$sub_field::random(&mut rand), $sub_field::random(rand)])
            }

            // TODO should be optimized
            fn double(self) -> Self {
                self + self
            }

            fn square(self) -> Self {
                self * self
            }

            fn double_assign(&mut self) {
                *self += self.clone()
            }

            fn square_assign(&mut self) {
                *self *= self.clone()
            }
        }

        impl PartialOrd for $extention_field {
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

        impl Ord for $extention_field {
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
macro_rules! extention_field_built_in {
    ($extention_field:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $extention_field {}

        impl Basic for $extention_field {}

        impl Default for $extention_field {
            fn default() -> Self {
                Self::zero()
            }
        }

        impl Display for $extention_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0[0].0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " + 0x")?;
                for i in self.0[1].0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, "*u")?;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! const_extention_field_operation {
    ($extention_field:ident, $sub_field:ident) => {
        impl $extention_field {
            pub const fn zero() -> Self {
                Self([$sub_field::zero(), $sub_field::zero()])
            }

            pub const fn one() -> Self {
                Self([$sub_field::one(), $sub_field::zero()])
            }
        }
    };
}

pub use {const_extention_field_operation, extention_field_built_in, extention_field_operation};
