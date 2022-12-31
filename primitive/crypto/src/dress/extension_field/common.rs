#[macro_export]
macro_rules! extension_field_built_in {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        const_extension_field_operation!($extension_field, $sub_field, $limbs_length);
        construct_extension_field!($extension_field, $sub_field, $limbs_length);

        impl ExtensionField for $extension_field {
            fn mul_by_nonresidue(self) -> Self {
                self.mul_by_nonres()
            }
        }

        impl ParityCmp for $extension_field {}

        impl Basic for $extension_field {}

        impl Default for $extension_field {
            fn default() -> Self {
                Self::zero()
            }
        }

        impl Display for $extension_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                for i in self.0.iter().rev() {
                    write!(f, "0x")?;
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }

        impl LowerHex for $extension_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                for i in self.0.iter().rev() {
                    write!(f, "0x")?;
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! const_extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        impl $extension_field {
            pub const fn zero() -> Self {
                Self([$sub_field::zero(); $limbs_length])
            }

            pub const fn one() -> Self {
                let mut limbs = [$sub_field::zero(); $limbs_length];
                limbs[0] = $sub_field::one();
                Self(limbs)
            }

            pub const fn dummy() -> Self {
                unimplemented!()
            }
        }
    };
}

#[macro_export]
macro_rules! construct_extension_field {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        #[derive(Debug, Clone, Copy, Decode, Encode)]
        pub struct $extension_field(pub(crate) [$sub_field; $limbs_length]);
    };
}

#[macro_export]
macro_rules! common_extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Div for $extension_field {
            type Output = $extension_field;

            #[inline]
            fn div(self, rhs: $extension_field) -> $extension_field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl<'a, 'b> Div<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn div(self, rhs: &'b $extension_field) -> $extension_field {
                let inv = rhs.invert().unwrap();
                self * &inv
            }
        }

        #[allow(clippy::suspicious_op_assign_impl)]
        impl DivAssign for $extension_field {
            fn div_assign(&mut self, rhs: $extension_field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }

        impl PrimeField for $extension_field {
            // wrong if this is problem
            const MODULUS: $extension_field = $extension_field::dummy();

            const INV: u64 = $sub_field::INV;

            fn from_u64(val: u64) -> Self {
                unimplemented!()
            }

            fn to_bits(self) -> Bits {
                unimplemented!()
            }

            fn is_zero(self) -> bool {
                let mut acc = true;
                self.0.iter().for_each(|a| acc = acc && a.is_zero());
                acc
            }

            // TODO should be optimized
            fn double(self) -> Self {
                self + self
            }

            fn square(self) -> Self {
                self.square_ext_field()
            }

            fn double_assign(&mut self) {
                *self += self.clone()
            }

            fn square_assign(&mut self) {
                *self *= self.clone()
            }
        }

        impl PartialOrd for $extension_field {
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

        impl Ord for $extension_field {
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

pub use {
    common_extension_field_operation, const_extension_field_operation, construct_extension_field,
    extension_field_built_in,
};
