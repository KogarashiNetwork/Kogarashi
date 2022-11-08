#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident) => {
        group_operation!($field, $p, $g, $e);

        ring_operation!($field, $p);
    };
}

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $i:ident) => {
        field_operation!($field, $p, $g, $e);

        built_in_operation!($field);

        impl PrimeField for $field {
            const INV: Self = $i;
        }
    };
}

#[macro_export]
macro_rules! fft_field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $i:ident, $r:ident) => {
        prime_field_operation!($field, $p, $g, $e, $i);

        impl FftField for $field {
            const ROOT_OF_UNITY: Self = $r;
        }
    };
}

pub use field_operation;

pub use prime_field_operation;

pub use fft_field_operation;
