use crate::arithmetic::{add, double, mul, sub};
use core::cmp::Ordering;
use parity_scale_codec::{Decode, Encode};

pub(crate) const MODULUS: &[u64; 4] = &[
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

pub(crate) const INV: u64 = 0x1ba3a358ef788ef9;

#[derive(Debug, Clone, Decode, Encode)]
pub(crate) struct Fr(pub(crate) [u64; 4]);

impl Fr {
    #[inline(always)]
    pub fn add_asign(&mut self, other: Self) {
        self.0 = add(&self.0, &other.0);
    }

    #[inline(always)]
    pub fn sub_assign(&mut self, other: Self) {
        self.0 = sub(&self.0, &other.0);
    }

    #[inline(always)]
    pub fn double_assign(&mut self) {
        self.0 = double(&self.0)
    }

    #[inline(always)]
    pub fn mul_assign(&mut self, other: Self) {
        self.0 = mul(&self.0, &other.0)
    }
}

impl Eq for Fr {}

impl PartialEq for Fr {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
            && self.0[1] == other.0[1]
            && self.0[2] == other.0[2]
            && self.0[3] == other.0[3]
    }
}

impl PartialOrd for Fr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[3 - i] != other.0[3 - i] {
                return self.0[3 - i] < other.0[3 - i];
            }
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[3 - i] != other.0[3 - i] {
                return self.0[3 - i] < other.0[3 - i];
            }
        }
        true
    }

    fn gt(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[3 - i] != other.0[3 - i] {
                return self.0[3 - i] > other.0[3 - i];
            }
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[3 - i] != other.0[3 - i] {
                return self.0[3 - i] > other.0[3 - i];
            }
        }
        true
    }
}

impl Ord for Fr {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.0.len() {
            if self.0[3 - i] != other.0[3 - i] {
                return if self.0[3 - i] > other.0[3 - i] {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
        }
        Ordering::Equal
    }
}
