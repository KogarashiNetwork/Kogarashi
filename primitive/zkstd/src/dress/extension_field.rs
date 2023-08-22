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

        impl ExtensionField for $extension_field {
            fn mul_by_nonresidue(self) -> Self {
                self.mul_by_nonres()
            }
        }
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

        impl RefOps for $extension_field {}

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

            // TODO should be optimized
            fn double(self) -> Self {
                self + self
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
