use crate::error::Error;
use bls_12_381::Fr;
use serde::{Deserialize, Serialize};
use zkstd::arithmetic::bits_256::*;
use zkstd::common::*;
use zkstd::dress::field::*;

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

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fp(pub(crate) [u64; 4]);

impl SigUtils<32> for Fp {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let tmp = self.montgomery_reduce();

        let mut res = [0; Self::LENGTH];
        res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
        res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
        res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
        res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

        res
    }

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        // SBP-M1 review: apply proper error handling instead of `unwrap`
        let l0 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let l1 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let l2 = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let l3 = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        let (_, borrow) = sbb(l0, MODULUS[0], 0);
        let (_, borrow) = sbb(l1, MODULUS[1], borrow);
        let (_, borrow) = sbb(l2, MODULUS[2], borrow);
        let (_, borrow) = sbb(l3, MODULUS[3], borrow);

        if borrow & 1 == 1 {
            Some(Self([l0, l1, l2, l3]) * Self(R2))
        } else {
            Some(Self([l0, l1, l2, l3]).reduce() * Self(R2))
        }
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

    pub fn reduce(&self) -> Self {
        Self(self.montgomery_reduce())
    }

    pub fn is_even(&self) -> bool {
        self.0[0] % 2 == 0
    }

    pub fn from_hash(hash: &[u8; 64]) -> Self {
        let d0 = Fp([
            u64::from_le_bytes(hash[0..8].try_into().unwrap()),
            u64::from_le_bytes(hash[8..16].try_into().unwrap()),
            u64::from_le_bytes(hash[16..24].try_into().unwrap()),
            u64::from_le_bytes(hash[24..32].try_into().unwrap()),
        ]);
        let d1 = Fp([
            u64::from_le_bytes(hash[32..40].try_into().unwrap()),
            u64::from_le_bytes(hash[40..48].try_into().unwrap()),
            u64::from_le_bytes(hash[48..56].try_into().unwrap()),
            u64::from_le_bytes(hash[56..64].try_into().unwrap()),
        ]);
        d0 * Fp(R2) + d1 * Fp(R3)
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
        let bls_scalar = Fr::from_bytes(scalar.to_bytes());

        assert!(
            bls_scalar.is_some(),
            "Failed to convert a Scalar from JubJub to BLS"
        );

        bls_scalar.unwrap()
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
