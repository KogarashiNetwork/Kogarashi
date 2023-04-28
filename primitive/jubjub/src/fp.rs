use crate::error::Error;
use dusk_bytes::Serializable;
use serde::{Deserialize, Serialize};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zero_bls12_381::Fr;
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fp(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

const GENERATOR: [u64; 4] = [2, 0, 0, 0];

// weird if this is problem
const MULTIPLICATIVE_GENERATOR: Fp = Fp([2, 0, 0, 0]);

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

fft_field_operation!(
    Fp,
    MODULUS,
    GENERATOR,
    MULTIPLICATIVE_GENERATOR,
    INV,
    ROOT_OF_UNITY,
    R,
    R2,
    R3,
    S
);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

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
}

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

pub fn compute_windowed_naf<F: FftField>(scalar: F, width: u8) -> [i8; 256] {
    let mut k = scalar.reduce();
    let mut i = 0;
    let one = F::one().reduce();
    let mut res = [0i8; 256];

    while k >= one {
        if !k.is_even() {
            let ki = k.mods_2_pow_k(width);
            res[i] = ki;
            let k_ = match (ki >= 0, ki < 0) {
                (true, false) => F::from([ki.unsigned_abs() as u64, 0u64, 0u64, 0u64]),
                (false, true) => -F::from([ki.unsigned_abs() as u64, 0u64, 0u64, 0u64]),
                (_, _) => unreachable!(),
            };
            k -= k_;
        } else {
            res[i] = 0i8;
        };

        k.divn(1u32);
        i += 1;
    }
    res
}

impl Fp {
    /// Reduces bit representation of numbers, such that
    /// they can be evaluated in terms of the least significant bit.
    pub fn reduce(&self) -> Self {
        Self(self.montgomery_reduce())
    }

    /// Evaluate if a `Scalar, from Fr` is even or not.
    pub fn is_even(&self) -> bool {
        self.0[0] % 2 == 0
    }

    /// Compute the result from `Scalar (mod 2^k)`.
    ///
    /// # Panics
    ///
    /// If the given k is > 32 (5 bits) as the value gets
    /// greater than the limb.  
    pub fn mod_2_pow_k(&self, k: u8) -> u8 {
        (self.0[0] & ((1 << k) - 1)) as u8
    }

    /// Compute the result from `Scalar (mods k)`.
    ///
    /// # Panics
    ///
    /// If the given `k > 32 (5 bits)` || `k == 0` as the value gets
    /// greater than the limb.   
    pub fn mods_2_pow_k(&self, w: u8) -> i8 {
        assert!(w < 32u8);
        let modulus = self.mod_2_pow_k(w) as i8;
        let two_pow_w_minus_one = 1i8 << (w - 1);

        match modulus >= two_pow_w_minus_one {
            false => modulus,
            true => modulus - ((1u8 << w) as i8),
        }
    }
}

impl From<i8> for Fp {
    fn from(val: i8) -> Fp {
        match (val >= 0, val < 0) {
            (true, false) => Fp([val.unsigned_abs() as u64, 0u64, 0u64, 0u64]),
            (false, true) => -Fp([val.unsigned_abs() as u64, 0u64, 0u64, 0u64]),
            (_, _) => unreachable!(),
        }
    }
}

impl From<Fp> for Fr {
    fn from(scalar: Fp) -> Fr {
        let bls_scalar = Fr::from_bytes(&scalar.to_bytes());

        // The order of a JubJub's Scalar field is shorter than a BLS Scalar,
        // so convert any jubjub scalar to a BLS' Scalar should always be
        // safe.
        assert!(
            bls_scalar.is_ok(),
            "Failed to convert a Scalar from JubJub to BLS"
        );

        bls_scalar.unwrap()
    }
}

impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl Serializable<32> for Fp {
    type Error = Error;

    /// Converts an element of `Self` into a byte representation in
    /// little-endian byte order.
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        // Turn into canonical form by computing
        // (a.R) / R = a
        let tmp = self.montgomery_reduce();

        let mut res = [0; Self::SIZE];
        res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
        res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
        res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
        res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

        res
    }

    /// Attempts to convert a little-endian byte representation of
    /// a field element into an element of `Self`, failing if the input
    /// is not canonical (is not smaller than r).
    fn from_bytes(bytes: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let mut tmp = Self([0, 0, 0, 0]);

        tmp.0[0] = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        tmp.0[1] = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        tmp.0[2] = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        tmp.0[3] = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        // Try to subtract the modulus
        let (_, borrow) = sbb(tmp.0[0], MODULUS[0], 0);
        let (_, borrow) = sbb(tmp.0[1], MODULUS[1], borrow);
        let (_, borrow) = sbb(tmp.0[2], MODULUS[2], borrow);
        let (_, borrow) = sbb(tmp.0[3], MODULUS[3], borrow);

        // If the element is smaller than MODULUS then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        if is_some == 0 {
            return Err(Error::BytesInvalid);
        }

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp *= Self(R2);

        Ok(tmp)
    }
}
