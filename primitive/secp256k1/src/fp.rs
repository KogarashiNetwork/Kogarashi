use serde::{Deserialize, Serialize};
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fp(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xfffffffefffffc2f,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
];

const GENERATOR: [u64; 4] = [7, 0, 0, 0];

/// R = 2^256 mod p
const R: [u64; 4] = [
    0x00000001000003d1,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

/// R^2 = 2^512 mod p
const R2: [u64; 4] = [
    0x000007a2000e90a1,
    0x0000000000000001,
    0x0000000000000000,
    0x0000000000000000,
];

/// R^3 = 2^768 mod p
const R3: [u64; 4] = [
    0x002bb1e33795f671,
    0x0000000100000b73,
    0x0000000000000000,
    0x0000000000000000,
];

const INV: u64 = 0xd838091dd2253531;

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

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

    #[test]
    fn mul_test() {
        let a = Fp::to_mont_form([u64::MAX, u64::MAX, 1, 1]);
        let b = Fp::to_mont_form([1, u64::MAX, u64::MAX, 1]);
        let c = Fp::to_mont_form([1, u64::MAX, 1, 1]);

        // a * b + a * c
        let ab = a * b;
        println!("{:?}", ab);
        let ac = a * c;
        println!("{:?}", ac);
        let d = ab + ac;
        println!("{:?}", d);

        // a * (b + c)
        let bc = b + c;
        let e = a * bc;

        assert_eq!(d, e);
    }
}
