use crate::error::Error;
use serde::{Deserialize, Serialize};
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Debug, Clone, Copy, Decode, Encode, Serialize, Deserialize)]

pub struct Fp(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

const GENERATOR: [u64; 4] = [2, 0, 0, 0];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

const INV: u64 = 0x1ba3a358ef788ef9;

const S: usize = 1;

const ROOT_OF_UNITY: Fp = Fp([
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
]);

fft_field_operation!(Fp, MODULUS, GENERATOR, INV, ROOT_OF_UNITY, R, R2, R3, S);

impl Fp {
    pub const fn to_mont_form(val: [u64; 4]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }

    pub fn from_hex(hex: &str) -> Result<Fp, Error> {
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
            limbs[i] = Fp::bytes_to_u64(&hex[i]).unwrap();
        }
        Ok(Fp(mul(limbs, R2, MODULUS, INV)))
    }

    fn as_bytes(&self) -> [u8; 64] {
        let mut bytes: [u8; 64] = [0; 64];
        let mut index = 15;
        for i in 0..self.0.len() {
            let mut number = self.0[i];
            for n in 0..16 {
                let quotient = number as u128 / 16_u128.pow(15 - n as u32);
                bytes[index - n] = quotient as u8;
                number = (number as u128 % 16_u128.pow(15 - n as u32)) as u64;
            }
            index += 16;
        }
        bytes
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
    use paste::paste;
    use rand_core::OsRng;
    use zero_crypto::dress::field::field_test;

    field_test!(fp_field, Fp, 1000);

    #[test]
    fn test_from_hex() {
        let a = Fp::from_hex("0x64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab")
            .unwrap();
        assert_eq!(
            a,
            Fp([
                0x4ddc8f91e171cd75,
                0x9b925835a7d203fb,
                0x0cdb538ead47e463,
                0x01a19f85f00d79b8,
            ])
        )
    }

    #[test]
    fn test_cmp() {
        let a = Fp::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad82")
            .unwrap();
        let b = Fp::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad83")
            .unwrap();

        assert!(a <= a);
        assert!(a >= a);
        assert!(a == a);
        assert!(a < b);
        assert!(a <= b);
        assert!(a != b);
    }
}
