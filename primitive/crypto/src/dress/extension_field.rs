mod common;
mod group;
mod ring;

pub use common::*;
pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        extension_field_ring_operation!($extension_field, $sub_field, $limbs_length);
        extension_field_built_in!($extension_field, $sub_field, $limbs_length);
        common_extension_field_operation!($extension_field, $sub_field, $limbs_length);

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
    };
}

pub use extension_field_operation;
