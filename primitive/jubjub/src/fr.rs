use crate::arithmetic::limbs::{add, double, mont, mul, neg, square, sub};
use crate::coordinate::Projective;
use crate::domain::field::field_operation;
use crate::error::Error;
use crate::interface::coordinate::Coordinate;
use core::{
    cmp::Ordering,
    fmt::{Binary, Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Neg, Sub},
    ops::{AddAssign, MulAssign, SubAssign},
};
use parity_scale_codec::{Decode, Encode};
use rand_core::RngCore;
use sp_std::vec::Vec;

pub(crate) const MODULUS: FrRaw = FrRaw([
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
]);

/// R = 2^256 mod r
const R: FrRaw = FrRaw([
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
]);

/// R^2 = 2^512 mod r
const R2: FrRaw = FrRaw([
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
]);

/// R^3 = 2^768 mod r
const R3: FrRaw = FrRaw([
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
]);

pub(crate) const INV: u64 = 0x1ba3a358ef788ef9;

const S: u32 = 1;

const ROOT_OF_UNITY: FrRaw = FrRaw([
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
]);

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, Decode, Encode)]
pub struct FrRaw([u64; 4]);

impl Ord for FrRaw {
    #[inline(always)]
    fn cmp(&self, other: &FrRaw) -> Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            match a.cmp(b) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => continue,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for FrRaw {
    #[inline(always)]
    fn partial_cmp(&self, other: &FrRaw) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u32> for FrRaw {
    fn from(x: u32) -> Self {
        let mut raw = Self::default();
        raw.0[0] = x as u64;
        raw
    }
}

impl From<u64> for FrRaw {
    fn from(x: u64) -> Self {
        let mut raw = Self::default();
        raw.0[0] = x;
        raw
    }
}

impl From<[u64; 4]> for FrRaw {
    fn from(x: [u64; 4]) -> Self {
        Self(x)
    }
}

impl From<Fr> for FrRaw {
    fn from(fr: Fr) -> Self {
        fr.into_raw()
    }
}

impl Display for FrRaw {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "0x")?;
        for i in self.0.iter().rev() {
            write!(f, "{:016x}", *i)?;
        }
        Ok(())
    }
}

impl FrRaw {
    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&e| e == 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
pub struct Fr(pub FrRaw);

field_operation!(Fr, MODULUS);

impl Fr {
    pub fn from_raw(raw: FrRaw) -> Result<Self, Error> {
        if raw < MODULUS {
            Ok(Fr(FrRaw(mul(&raw.0, &R2.0, &MODULUS.0))))
        } else {
            Err(Error::OutOfField)
        }
    }

    pub fn into_raw(&self) -> FrRaw {
        FrRaw(mont(
            &[
                self.0 .0[0],
                self.0 .0[1],
                self.0 .0[2],
                self.0 .0[3],
                0,
                0,
                0,
                0,
            ],
            &MODULUS.0,
        ))
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
        Ok(Fr(FrRaw(mul(&limbs, &R2.0, &MODULUS.0))))
    }

    fn as_bytes(&self) -> [u8; 64] {
        let mut bytes: [u8; 64] = [0; 64];
        let mut index = 15;
        for i in 0..self.0 .0.len() {
            let mut number = self.0 .0[i];
            for n in 0..16 {
                let quotient = number as u128 / 16_u128.pow(15 - n as u32);
                bytes[index - n] = quotient as u8;
                number = (number as u128 % 16_u128.pow(15 - n as u32)) as u64;
            }
            index += 16;
        }
        bytes
    }

    fn as_bits(&self) -> [u8; 256] {
        let mut index = 256;
        let mut bits: [u8; 256] = [0; 256];
        for mut x in self.0 .0 {
            for _ in 0..64 {
                index -= 1;
                bits[index] = (x & 1) as u8;
                x >>= 1;
            }
        }
        bits
    }

    fn from_u512(limbs: [u64; 8]) -> Self {
        let a = mul(&[limbs[0], limbs[1], limbs[2], limbs[3]], &R2.0, &MODULUS.0);
        let b = mul(&[limbs[4], limbs[5], limbs[6], limbs[7]], &R3.0, &MODULUS.0);
        let c = add(&a, &b, &MODULUS.0);
        Fr(FrRaw(c))
    }

    fn bytes_to_u64(bytes: &[u8; 16]) -> Result<u64, Error> {
        let mut res: u64 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            res += match byte {
                0..=15 => 16u64.pow(i as u32) * (*byte as u64),
                _ => return Err(Error::BytesInvalid),
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    #[test]
    fn test_is_zero() {
        let fr = Fr(FrRaw([0, 0, 0, 0]));
        assert!(fr.is_zero());
        let fr = Fr(FrRaw([0, 0, 0, 1]));
        assert!(!fr.is_zero());
    }

    #[test]
    fn test_fmt_and_to_bin() {
        let _fr = Fr(FrRaw([
            0xd0970e5ed6f72cb7,
            0xa6682093ccc81082,
            0x06673b0101343b00,
            0x0e7db4ea6533afa9,
        ]));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        #[test]
        fn test_binary_method(x in any::<u16>()) {
            let fr = Fr(FrRaw::from(x as u64));
            let g = Projective::generator();
            let mul = fr * g.clone();
            let rev_mul = g.clone() * fr;
            assert_eq!(mul, rev_mul);

            let mut acc = Projective::identity();
            for _ in 0..x {
                acc += g.clone();
            }

            assert_eq!(acc, mul);
        }
    }

    // TODO
    #[test]
    fn negative_binary_method() {
        let a = Fr(FrRaw::from(10_u64));
        let b = Fr(FrRaw::from(20_u64));
        // A - B
        // Raw: 0x08d2ac6947db3e87379d9f244f3d452dc40c70802e393ae15121322dc385917b
        // Mont: 0x0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cad
        let _x = (a - b) * Projective::generator();
        let _y = a * Projective::generator() + (b * Projective::generator()).neg();
        // println!("{} {} {}", y.x, y.y, y.z);
        // assert_eq!(x, y);
    }

    #[test]
    fn test_from_hex() {
        let a = Fr::from_hex("0x64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab")
            .unwrap();
        assert_eq!(
            a,
            Fr(FrRaw([
                0x4ddc8f91e171cd75,
                0x9b925835a7d203fb,
                0x0cdb538ead47e463,
                0x01a19f85f00d79b8,
            ]))
        )
    }

    #[test]
    fn test_cmp() {
        let a = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad82")
            .unwrap();
        let b = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad83")
            .unwrap();

        assert!(a <= a);
        assert!(a >= a);
        assert!(a == a);
        assert!(a < b);
        assert!(a <= b);
        assert!(a != b);
    }

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn test_invert(x in arb_fr()) {
            match Fr::_invert(&x) {
                Some(y) => {
                    let z = mul(&x.0.0, &y, &MODULUS.0);
                    assert_eq!(Fr(FrRaw(z)), Fr::one());
                },
                None => assert_eq!(x, Fr::zero())
            }

        }
    }

    #[test]
    fn test_montgomery_form() {
        assert!(Fr::from_raw(MODULUS).is_err());
        let a = FrRaw([
            0x302ac6aa9b26189e,
            0x4ac882606b48941c,
            0xa43a060ccb89ddd7,
            0x03fb5f814d8da217,
        ]);
        let a_fr = Fr::from_raw(a).unwrap();
        let b = FrRaw([
            0xb3b4ffd8ef558c79,
            0xb66914dbf986df8f,
            0x0397e6ddcd846f14,
            0x023ae83ad1c563a5,
        ]);
        let b_fr = Fr::from_raw(b).unwrap();
        let c = FrRaw([
            0x37ca877d339e33c0,
            0xb495f8d0546afd2d,
            0x76a419ae0060413b,
            0x017cefddee6b2200,
        ]);
        assert_eq!((a_fr * b_fr).into_raw(), c);

        assert!(Fr::from_raw(FrRaw::from(0_u64)).unwrap().is_zero());
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn test_from_mont_and_back(a in arb_fr()) {
            let a_raw = a.into_raw();
            let a1_raw = FrRaw::from(a);
            assert_eq!(a_raw, a1_raw);
            let a_again = Fr::from_raw(a_raw).unwrap();
            assert_eq!(a, a_again);
        }
    }
}
