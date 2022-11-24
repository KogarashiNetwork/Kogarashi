mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! extention_field_operation {
    ($extention_field:ident, $sub_field:ident, $g:ident) => {
        extention_field_ring_operation!($extention_field, $g);

        extention_field_built_in!($extention_field);

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
                self.0 = $extention_field([re, im])
            }
        }

        impl PrimeField for $extention_field {
            const MODULUS: $sub_field = $sub_field::MODULUS;

            const INV: u64 = $sub_field::INV;

            fn from_u64(val: u64) -> Self {
                unimplemented!()
            }

            fn to_bits(self) -> Bits {
                unimplemented!()
            }

            fn is_zero(self) -> bool {
                self.0[0].is_zero & self.0[1].is_zero
            }

            fn random(rand: impl RngCore) -> Self {
                [$sub_field::random(rand), $sub_field::random(rand)]
            }

            fn double(self) -> Self {
                self + self
            }

            fn square(self) -> Self {
                self * self
            }

            fn double_assign(&mut self) {
                self += self
            }

            fn square_assign(&mut self) {
                self *= self
            }
        }

        impl $extention_field {
            pub const fn zero() -> $extention_field {
                $extention_field([$sub_field::zero(), $sub_field::zero()])
            }

            pub const fn one() -> $extention_field {
                $extention_field([$sub_field::one(), $sub_field::zero()])
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

pub use {extention_field_built_in, extention_field_operation};
