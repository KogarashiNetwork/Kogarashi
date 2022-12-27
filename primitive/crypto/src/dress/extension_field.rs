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
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                $extension_field([re, im])
            }
        }

        impl<'a, 'b> Mul<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn mul(self, rhs: &'b $extension_field) -> $extension_field {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                $extension_field([re, im])
            }
        }

        impl MulAssign for $extension_field {
            fn mul_assign(&mut self, rhs: $extension_field) {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1] + (self.0[1] * rhs.0[0]));
                self.0 = [re, im]
            }
        }
    };
}

#[macro_export]
macro_rules! higher_degree_extension_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        extension_field_ring_operation!($extension_field, $sub_field, $limbs_length);
        extension_field_built_in!($extension_field, $sub_field, $limbs_length);
        common_extension_field_operation!($extension_field, $sub_field, $limbs_length);

        impl Field for $extension_field {}

        impl Mul for $extension_field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $extension_field) -> Self {
                todo!()
            }
        }

        impl<'a, 'b> Mul<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn mul(self, rhs: &'b $extension_field) -> $extension_field {
                todo!()
            }
        }

        impl MulAssign for $extension_field {
            fn mul_assign(&mut self, rhs: $extension_field) {
                todo!()
            }
        }
    };
}

pub use {extension_field_operation, higher_degree_extension_field_operation};
