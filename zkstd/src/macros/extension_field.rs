mod field;
mod group;
mod ring;

pub use field::*;
pub use group::*;
pub use ring::*;

/// extension field basic operation macro
#[macro_export]
macro_rules! extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        prime_extension_field_operation!($extension_field, $sub_field, $limbs_length);

        /// extension field of base field
        #[derive(Clone, Copy, Decode, Encode)]
        pub struct $extension_field(pub(crate) [$sub_field; $limbs_length]);

        impl ParityCmp for $extension_field {}
        impl Basic for $extension_field {}
    };
}

/// prime field operation for extension field macro
#[macro_export]
macro_rules! prime_extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        ext_field_operation!($extension_field, $sub_field, $limbs_length);

        impl From<u64> for $extension_field {
            fn from(val: u64) -> $extension_field {
                unimplemented!()
            }
        }

        impl PrimeField for $extension_field {
            // wrong if this is problem
            const MODULUS: $extension_field = $extension_field::one();

            const INV: u64 = $sub_field::INV;

            fn is_zero(self) -> bool {
                self.0.iter().all(|x| x.is_zero())
            }

            fn to_bits(self) -> Bits {
                unimplemented!()
            }

            fn to_nafs(self) -> Nafs {
                unimplemented!()
            }

            fn to_raw_bytes(&self) -> Vec<u8> {
                unimplemented!()
            }

            fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
                unimplemented!()
            }

            fn pow_of_2(by: u64) -> Self {
                unimplemented!()
            }

            fn double(self) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i].double();
                }
                $extension_field(limbs)
            }

            fn square(self) -> Self {
                self.square_ext_field()
            }

            fn double_assign(&mut self) {
                *self = self.double()
            }

            fn square_assign(&mut self) {
                *self = self.square()
            }
        }
    };
}

pub use {extension_field_operation, prime_extension_field_operation};
