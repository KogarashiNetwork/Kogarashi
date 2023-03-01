use serde::{Deserialize, Serialize};
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fp(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xffffffffffffffed,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
];

const R: [u64; 4] = [
    0x0000000000000026,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

const R2: [u64; 4] = [
    0x00000000000005a4,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

const R3: [u64; 4] = [
    0x000000000000d658,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

const INV: u64 = 0x86bca1af286bca1b;

const GENERATOR: [u64; 4] = [7, 0, 0, 0];

impl Fp {
    pub const fn to_mont_form(val: [u64; 4]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }

    pub(crate) const fn montgomery_reduce(self) -> [u64; 4] {
        mont(
            [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
            MODULUS,
            INV,
        )
    }
}

prime_field_operation!(Fp, MODULUS, GENERATOR, INV, R, R2, R3);

impl From<u64> for Fp {
    fn from(val: u64) -> Fp {
        Fp(from_u64(val, R2, MODULUS, INV))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

    field_test!(fp_field, Fp, 1000);
}
