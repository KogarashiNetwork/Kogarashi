use crate::arithmetic::{add, double, mul, sub};
use crate::error::Error;
use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
};
use parity_scale_codec::{Decode, Encode};
use rand_core::RngCore;

#[allow(unused_imports)]
use libc_print::libc_println as println;

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

    pub const fn zero() -> Fr {
        Fr([0, 0, 0, 0])
    }

    fn from_hex(hex: &str) -> Result<Fr, Error> {
        let max_len = 64;
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let length = hex.len();
        if length > max_len {
            return Err(Error::HexStringTooLong);
        }
        let hex_bytes = hex.as_bytes();
        Fr::from_bytes(hex_bytes)
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

    fn from_bytes(bytes: &[u8]) -> Result<Fr, Error> {
        let max_len = 64;
        let length = bytes.len();
        if length > max_len {
            return Err(Error::BytesTooLong);
        }
        let mut hex: [[u8; 16]; 4] = [[0; 16]; 4];
        for i in 0..max_len {
            hex[i / 16][i % 16] = if i >= length {
                0
            } else {
                match bytes[length - i - 1] {
                    48..=57 => bytes[length - i - 1] - 48,
                    65..=70 => bytes[length - i - 1] - 55,
                    97..=102 => bytes[length - i - 1] - 87,
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

    pub fn random(mut rand: impl RngCore) -> Result<Self, Error> {
        let mut random_bytes = [0; 64];
        rand.fill_bytes(&mut random_bytes[..]);
        Fr::from_bytes(&random_bytes)
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

impl Default for Fr {
    fn default() -> Self {
        Fr::zero()
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
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a != b {
                return a < b;
            }
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a != b {
                return a < b;
            }
        }
        true
    }

    fn gt(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a != b {
                return a > b;
            }
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a != b {
                return a > b;
            }
        }
        true
    }
}

impl Ord for Fr {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a < b {
                return Ordering::Less;
            } else if a > b {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl Display for Fr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let tmp = self.to_bytes();
        write!(f, "0x")?;
        for &b in tmp.iter().rev() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
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
            Fr::random(rng).unwrap();
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
        b.add_assign(Fr(*MODULUS));
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
        b.sub_assign(Fr(*MODULUS));
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
