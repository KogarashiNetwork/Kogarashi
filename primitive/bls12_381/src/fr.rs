use core::borrow::Borrow;
use core::iter::{Product, Sum};
use serde::{Deserialize, Serialize};
use zkstd::arithmetic::bits_256::*;
use zkstd::common::*;
use zkstd::dress::field::*;

use crate::error::Error;

const MODULUS: [u64; 4] = [
    0xffffffff00000001,
    0x53bda402fffe5bfe,
    0x3339d80809a1d805,
    0x73eda753299d7d48,
];

const GENERATOR: [u64; 4] = [
    0x0000000efffffff1,
    0x17e363d300189c0f,
    0xff9c57876f8457b0,
    0x351332208fc5a8c4,
];

/// generator of the scalar field
pub const MULTIPLICATIVE_GENERATOR: Fr = Fr([7, 0, 0, 0]);

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x00000001fffffffe,
    0x5884b7fa00034802,
    0x998c4fefecbc4ff5,
    0x1824b159acc5056f,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0xc999e990f3f29c6d,
    0x2b6cedcb87925c23,
    0x05d314967254398f,
    0x0748d9d99f59ff11,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xc62c1807439b73af,
    0x1b3e0d188cf06990,
    0x73d13c71c7b5f418,
    0x6e2a5bb9c8db33e9,
];

pub const INV: u64 = 0xfffffffeffffffff;

const S: usize = 32;

/// multiplicative group generator of n th root of unity
pub const ROOT_OF_UNITY: Fr = Fr([
    0xb9b58d8c5f0e466a,
    0x5b1b4c801819d7ec,
    0x0af53ae352a31e64,
    0x5bf3adda19e9b27b,
]);

pub const TWO_ADACITY: u32 = 32;

/// Bls12 381 curve scalar field
#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fr(pub [u64; 4]);

impl SigUtils<32> for Fr {
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
            None
        }
    }
}

impl Fr {
    pub const fn to_mont_form(val: [u64; 4]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }

    pub const fn inner(&self) -> [u64; 4] {
        self.0
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
        Ok(Fr(mul(limbs, R2, MODULUS, INV)))
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

    pub(crate) const fn montgomery_reduce(self) -> [u64; 4] {
        mont(
            [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
            MODULUS,
            INV,
        )
    }

    pub fn to_costomized_repr(self) -> Bits {
        let mut index = 128;
        let mut bits: [u8; 128] = [0; 128];
        for limb in self.montgomery_reduce() {
            for byte in limb.to_le_bytes().iter() {
                for i in 0..4 {
                    index -= 1;
                    bits[index] = (byte >> (i * 2)) & 3;
                }
            }
        }
        bits.into_iter()
            .skip_while(|w_bit| w_bit == &0)
            .collect::<Vec<_>>()
    }

    pub fn is_odd(self) -> bool {
        let raw = self.montgomery_reduce();
        (raw[0] % 2) != 0
    }

    pub fn pow_vartime(&self, by: &[u64; 4]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res.mul_assign(*self);
                }
            }
        }
        res
    }

    pub fn sqrt(&self) -> Option<Self> {
        let w = self.pow_vartime(&[
            0x7fff2dff7fffffff,
            0x04d0ec02a9ded201,
            0x94cebea4199cec04,
            0x39f6d3a9,
        ]);

        let mut v = Self::S;
        let mut x = w * self;
        let mut b = x * w;
        let mut z = Self::ROOT_OF_UNITY;

        for max_v in (1..=Self::S).rev() {
            let mut k = 1;
            let mut b2k = b.square();
            let mut j_less_than_v = true;

            for j in 2..max_v {
                j_less_than_v &= j != v;
                if b2k == Self::one() {
                    if j_less_than_v {
                        z.square_assign()
                    };
                } else {
                    b2k = b2k.square();
                    k = j;
                };
            }

            if b != Self::one() {
                x.mul_assign(z)
            };
            z.square_assign();
            b *= z;
            v = k;
        }

        if &x.square() == self {
            Some(x)
        } else {
            None
        }
    }
}

impl<'a, 'b> BitXor<&'b Fr> for &'a Fr {
    type Output = Fr;

    fn bitxor(self, rhs: &'b Fr) -> Fr {
        let a_red = self.montgomery_reduce();
        let b_red = rhs.montgomery_reduce();
        Fr::to_mont_form([
            a_red[0] ^ b_red[0],
            a_red[1] ^ b_red[1],
            a_red[2] ^ b_red[2],
            a_red[3] ^ b_red[3],
        ])
    }
}

impl BitXor<Fr> for Fr {
    type Output = Fr;

    fn bitxor(self, rhs: Fr) -> Fr {
        &self ^ &rhs
    }
}

impl<'a, 'b> BitAnd<&'b Fr> for &'a Fr {
    type Output = Fr;

    fn bitand(self, rhs: &'b Fr) -> Fr {
        let a_red = self.montgomery_reduce();
        let b_red = rhs.montgomery_reduce();
        Fr::to_mont_form([
            a_red[0] & b_red[0],
            a_red[1] & b_red[1],
            a_red[2] & b_red[2],
            a_red[3] & b_red[3],
        ])
    }
}

impl BitAnd<Fr> for Fr {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Fr {
        &self & &rhs
    }
}

fft_field_operation!(
    Fr,
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

impl<T> Product<T> for Fr
where
    T: Borrow<Fr>,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Fr::one(), |acc, item| acc * *item.borrow())
    }
}

impl<T> Sum<T> for Fr
where
    T: Borrow<Fr>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::zero(), |acc, item| acc + *item.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

    field_test!(bls12_381_scalar, Fr, 1000);

    #[test]
    fn test_root_of_unity() {
        let s = Fr::S;
        let mut root_of_unity = Fr::ROOT_OF_UNITY;
        (0..s).for_each(|_| root_of_unity.square_assign());
        assert_eq!(root_of_unity, Fr::one())
    }

    #[test]
    fn test_sqrt() {
        let mut square = Fr([
            0x46cd85a5f273077e,
            0x1d30c47dd68fc735,
            0x77f656f60beca0eb,
            0x494aa01bdf32468d,
        ]);

        let mut none_count = 0;

        for _ in 0..100 {
            let square_root = square.sqrt();
            if bool::from(square_root.is_none()) {
                none_count += 1;
            } else {
                assert_eq!(square_root.unwrap() * square_root.unwrap(), square);
            }
            square -= Fr::one();
        }

        assert_eq!(49, none_count);
    }

    #[test]
    fn test_serde() {
        for _ in 0..100000 {
            let s = Fr::random(OsRng);
            let bytes = s.to_bytes();
            let s_prime = Fr::from_bytes(bytes).unwrap();
            assert_eq!(s, s_prime);
        }
    }
}
