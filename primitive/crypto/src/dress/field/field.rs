#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        ring_operation!($field, $p, $g, $e, $r2, $r3, $inv);

        impl Field for $field {}

        impl Div for $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: $field) -> $field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        impl DivAssign for $field {
            fn div_assign(&mut self, rhs: $field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }
    };
}

pub use field_operation;
