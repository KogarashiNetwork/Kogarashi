#[macro_export]
macro_rules! ext_field_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        use zkstd::behave::*;
        use zkstd::common::*;

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

pub use ext_field_operation;
