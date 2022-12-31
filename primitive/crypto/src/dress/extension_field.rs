mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! ext_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        ext_field_ring_operation!($extension_field, $sub_field, $limbs_length);

        impl Field for $extension_field {}

        impl Div for $extension_field {
            type Output = $extension_field;

            #[inline]
            fn div(self, rhs: $extension_field) -> $extension_field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        impl DivAssign for $extension_field {
            fn div_assign(&mut self, rhs: $extension_field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }
    };
}

#[macro_export]
macro_rules! extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        prime_extention_field_operation!($extension_field, $sub_field, $limbs_length);
        field_built_in!($extension_field);

        #[derive(Debug, Clone, Copy, Decode, Encode)]
        pub struct $extension_field(pub(crate) [$sub_field; $limbs_length]);

        impl ExtensionField for $extension_field {
            fn mul_by_nonresidue(self) -> Self {
                self.mul_by_nonres()
            }
        }
    };
}

#[macro_export]
macro_rules! prime_extention_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        ext_field_operation!($extension_field, $sub_field, $limbs_length);

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
                self.0.iter().all(|x| x.is_zero())
            }

            // TODO should be optimized
            fn double(self) -> Self {
                self + self
            }

            fn square(self) -> Self {
                self.square_ext_field()
            }

            fn double_assign(&mut self) {
                *self += self.double()
            }

            fn square_assign(&mut self) {
                *self *= self.square()
            }
        }
    };
}

pub use {ext_field_operation, extension_field_operation, prime_extention_field_operation};
