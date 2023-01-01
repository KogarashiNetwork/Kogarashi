mod field;
mod group;
mod ring;

pub use field::*;
pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        field_operation!($field, $p, $g, $r, $inv, $r, $r2, $r3);
        field_built_in!($field);

        impl PrimeField for $field {
            const MODULUS: Self = $field($p);

            const INV: u64 = $inv;

            fn from_u64(val: u64) -> Self {
                Self(from_u64(val))
            }

            fn to_bits(self) -> Bits {
                to_bits(self.0)
            }

            fn is_zero(self) -> bool {
                self.0.iter().all(|x| *x == 0)
            }

            fn double(self) -> Self {
                Self(double(self.0, $p))
            }

            fn square(self) -> Self {
                Self(square(self.0, $p, $inv))
            }

            fn double_assign(&mut self) {
                self.0 = double(self.0, $p)
            }

            fn square_assign(&mut self) {
                self.0 = square(self.0, $p, $inv)
            }
        }
    };
}

#[macro_export]
macro_rules! fft_field_operation {
    ($field:ident, $p:ident, $g:ident, $i:ident, $u:ident, $r:ident, $r2:ident, $r3:ident, $s:ident) => {
        prime_field_operation!($field, $p, $g, $i, $r, $r2, $r3);

        impl FftField for $field {
            const S: usize = $s;

            const ROOT_OF_UNITY: Self = $u;

            fn pow(self, val: u64) -> Self {
                Self(pow(self.0, [val, 0, 0, 0], $r, $p, $i))
            }
        }

        impl From<u64> for $field {
            fn from(val: u64) -> $field {
                $field(mul(from_u64(val), $r2, $p, $i))
            }
        }

        impl ParallelCmp for $field {}
    };
}

#[macro_export]
macro_rules! pairing_field_operation {
    ($field:ident, $p:ident, $g:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        prime_field_operation!($field, $p, $g, $inv, $r, $r2, $r3);

        impl PairingField for $field {}
    };
}

pub use {fft_field_operation, pairing_field_operation, prime_field_operation};
