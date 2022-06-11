use crate::arithmetic::{add, double, mul, neg, square, sub};
use crate::dress::field_operation;
use crate::error::Error;
use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Neg, Sub},
    ops::{AddAssign, MulAssign, SubAssign},
};
use parity_scale_codec::{Decode, Encode};
use rand_core::RngCore;

pub(crate) const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: &[u64; 4] = &[
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: &[u64; 4] = &[
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

pub(crate) const INV: u64 = 0x1ba3a358ef788ef9;

const S: u32 = 1;

const ROOT_OF_UNITY: &[u64; 4] = &[
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
];

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fr(pub(crate) [u64; 4]);

field_operation!(Fr);

impl Fr {
    #[inline(always)]
    pub fn add_assign(&mut self, other: Self) {
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

    #[inline(always)]
    pub fn square_assign(&mut self) {
        self.0 = square(&self.0)
    }

    pub const fn zero() -> Fr {
        Fr([0, 0, 0, 0])
    }

    pub const fn one() -> Fr {
        Fr([1, 0, 0, 0])
    }

    pub fn from_hex(hex: &str) -> Result<Fr, Error> {
        let max_len = 64;
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let length = hex.len();
        if length > max_len {
            return Err(Error::HexStringTooLong);
        }
        let hex_bytes = hex.as_bytes();

        let mut hex: [[u8; 16]; 4] = [[0; 16]; 4];
        for i in 0..max_len {
            hex[i / 16][i % 16] = if i >= length {
                0
            } else {
                match hex_bytes[length - i - 1] {
                    48..=57 => hex_bytes[length - i - 1] - 48,
                    65..=70 => hex_bytes[length - i - 1] - 55,
                    97..=102 => hex_bytes[length - i - 1] - 87,
                    _ => return Err(Error::HexStringInvalid),
                }
            };
        }
        let mut limbs: [u64; 4] = [0; 4];
        for i in 0..hex.len() {
            limbs[i] = Fr::bytes_to_u64(&hex[i]).unwrap();
        }
        Ok(Fr(limbs))
    }

    fn to_bytes(&self) -> [u8; 64] {
        let mut bytes: [u8; 64] = [0; 64];
        let mut index = 0;
        for i in 0..self.0.len() {
            let mut number = self.0[i];
            for n in 0..16 {
                let quotient = number / 16_u64.pow(16 - n as u32);
                bytes[index] = quotient as u8;
                number = number % 16_u64.pow(16 - n as u32);
                index += 1;
            }
        }
        bytes
    }

    pub fn random(mut rand: impl RngCore) -> Self {
        Fr::from_u512([
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
        ])
    }

    fn from_u512(limbs: [u64; 8]) -> Self {
        let a = mul(&[limbs[0], limbs[1], limbs[2], limbs[3]], R2);
        let b = mul(&[limbs[4], limbs[5], limbs[6], limbs[7]], R3);
        let c = add(&a, &b);
        Fr(c)
    }

    fn bytes_to_u64(bytes: &[u8; 16]) -> Result<u64, Error> {
        let mut res: u64 = 0;
        for i in 0..bytes.len() {
            res += match bytes[i] {
                0..=15 => 16u64.pow(i as u32) * bytes[i] as u64,
                _ => return Err(Error::BytesInvalid),
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod fr_tests {
    use super::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    #[test]
    fn test_random() {
        for i in 0..10000 {
            let mut initial_seeds = [
                0x43, 0x62, 0xbe, 0x7d, 0x23, 0xad, 0x56, 0xcd, 0x33, 0x0a, 0x22, 0x23, 0x46, 0x36,
                0xac, 0xef,
            ];
            let seed = i as u8 % u8::MAX;
            let index = (seed % 16) as usize;
            initial_seeds[index] = seed;
            let rng = XorShiftRng::from_seed(initial_seeds);
            let randomness = Fr::random(rng);
            assert!(randomness < Fr(MODULUS))
        }
    }

    #[test]
    fn test_from_hex() {
        let a = Fr::from_hex("0x64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab")
            .unwrap();
        assert_eq!(
            a,
            Fr([
                0xb9feffffffffaaab,
                0x1eabfffeb153ffff,
                0x6730d2a0f6b0f624,
                0x64774b84f38512bf,
            ])
        )
    }

    #[test]
    fn test_add() {
        // a + 0 = a
        let mut a =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        a.add_assign(Fr::zero());
        assert_eq!(
            a,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap()
        );

        // a + modulus = a
        let mut b =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        b.add_assign(Fr(MODULUS));
        assert_eq!(
            b,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap()
        );

        // a + 1
        let mut c =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        c.add_assign(Fr([1, 0, 0, 0]));
        assert_eq!(
            c,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e70")
                .unwrap()
        )
    }

    #[test]
    fn test_sub() {
        // a - 0 = a
        let mut a =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        a.sub_assign(Fr::zero());
        assert_eq!(
            a,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap()
        );

        // a - modulus = a
        let mut b =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        b.sub_assign(Fr(MODULUS));
        assert_eq!(
            b,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap()
        );

        // a - 1
        let mut c =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        c.sub_assign(Fr([1, 0, 0, 0]));
        assert_eq!(
            c,
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6e")
                .unwrap()
        )
    }

    #[test]
    fn test_double() {
        // a double = a + a
        let mut a =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        a.double_assign();
        let mut b =
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap();
        b.add_assign(
            Fr::from_hex("0x0a85fa9c9fef6326f04bc41062fd73229abac9e4157b61727e7140b5196b9e6f")
                .unwrap(),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_mul() {
        let mut a = Fr([
            0xb433b01287f71744,
            0x4eafb86728c4d108,
            0xfdd52c14b9dfbe65,
            0x2ff1f3434821118,
        ]);
        a.mul_assign(Fr([
            0xdae00fc63c9fa90f,
            0x5a5ed89b96ce21ce,
            0x913cd26101bd6f58,
            0x3f0822831697fe9,
        ]));
        assert_eq!(
            a,
            Fr([
                0xb68ecb61d54d2992,
                0x5ff95874defce6a6,
                0x3590eb053894657d,
                0x53823a118515933
            ])
        );
    }

    #[test]
    fn test_square() {
        let mut a = Fr([
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xe7db4ea6533afa8,
        ]);
        let mut b = a.clone();
        let c = a.clone();
        a.square_assign();
        b.mul_assign(c);
        assert_eq!(a, b);
    }

    #[test]
    fn test_basic_arithmetic() {
        let a = Fr::one();
        let b = Fr::one();
        let c = a + b;
    }

    #[test]
    fn test_cmp() {
        let a = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad82")
            .unwrap();
        let b = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad83")
            .unwrap();

        assert_eq!(a <= a, true);
        assert_eq!(a >= a, true);
        assert_eq!(a == a, true);
        assert_eq!(a < b, true);
        assert_eq!(a > b, false);
        assert_eq!(a != b, true);
        assert_eq!(a == b, false);
    }
}
