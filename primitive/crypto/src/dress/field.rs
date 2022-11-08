mod group;
mod ring;

pub use group::*;
pub use ring::*;

pub use crate::arithmetic::{add, double, invert, mul, neg, square, sub};
pub use crate::behave::{Basic, FftField, Field, Group, ParallelCmp, ParityCmp, PrimeField, Ring};
pub use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Div, Mul, Neg, Sub},
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};
pub use parity_scale_codec::{Decode, Encode};

#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident) => {
        group_operation!($field, $p, $g, $e);

        ring_operation!($field, $p);

        impl Field for $field {}

        impl Div for $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: $field) -> $field {
                let inv = $field(invert(&rhs.0, &$p.0).unwrap());
                self * inv
            }
        }

        impl<'a, 'b> Div<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: &'b $field) -> $field {
                let inv = $field(invert(&rhs.0, &$p.0).unwrap());
                self * &inv
            }
        }

        impl DivAssign for $field {
            fn div_assign(&mut self, rhs: $field) {
                let inv = $field(invert(&rhs.0, &$p.0).unwrap());
                *self *= inv
            }
        }
    };
}

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $i:ident) => {
        field_operation!($field, $p, $g, $e);

        built_in_operation!($field);

        impl PrimeField for $field {
            const INV: u64 = $i;
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

        impl ParallelCmp for $field {}
    };
}

pub use field_operation;

pub use prime_field_operation;

pub use fft_field_operation;
