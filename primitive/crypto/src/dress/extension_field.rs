mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        extension_field_ring_operation!($extension_field, $sub_field, $limbs_length);
        field_built_in!($extension_field);

        #[derive(Debug, Clone, Copy, Decode, Encode)]
        pub struct $extension_field(pub(crate) [$sub_field; $limbs_length]);

        impl ExtensionField for $extension_field {
            fn mul_by_nonresidue(self) -> Self {
                self.mul_by_nonres()
            }
        }

        impl Field for $extension_field {}

        impl Mul for $extension_field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $extension_field) -> Self {
                self.mul_ext_field(rhs)
            }
        }

        impl<'a, 'b> Mul<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn mul(self, rhs: &'b $extension_field) -> $extension_field {
                self.mul_ext_field(*rhs)
            }
        }

        impl MulAssign for $extension_field {
            fn mul_assign(&mut self, rhs: $extension_field) {
                *self = self.mul_ext_field(rhs)
            }
        }

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
    };
}

pub use extension_field_operation;
