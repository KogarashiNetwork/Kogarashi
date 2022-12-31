#[macro_export]
macro_rules! extension_field_ring_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        extension_field_group_operation!($extension_field, $sub_field, $limbs_length);

        impl Ring for $extension_field {
            const MULTIPLICATIVE_IDENTITY: $extension_field = $extension_field::one();
        }
    };
}

pub use extension_field_ring_operation;
