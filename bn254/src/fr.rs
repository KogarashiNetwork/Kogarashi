use core::borrow::Borrow;
use core::iter::{Product, Sum};
use zkstd::arithmetic::bits_256::*;
use zkstd::common::*;
use zkstd::macros::field::*;

use crate::error::Error;

/// r = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
const MODULUS: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

const GENERATOR: [u64; 4] = [7, 0, 0, 0];

/// generator of the scalar field
pub const MULTIPLICATIVE_GENERATOR: Fr = Fr::to_mont_form([7, 0, 0, 0]);

/// `R = 2^256 mod r`
/// `0xe0a77c19a07df2f666ea36f7879462e36fc76959f60cd29ac96341c4ffffffb`
const R: [u64; 4] = [
    0xac96341c4ffffffb,
    0x36fc76959f60cd29,
    0x666ea36f7879462e,
    0x0e0a77c19a07df2f,
];

/// `R^2 = 2^512 mod r`
/// `0x216d0b17f4e44a58c49833d53bb808553fe3ab1e35c59e31bb8e645ae216da7`
const R2: [u64; 4] = [
    0x1bb8e645ae216da7,
    0x53fe3ab1e35c59e3,
    0x8c49833d53bb8085,
    0x0216d0b17f4e44a5,
];

/// `R^3 = 2^768 mod r`
/// `0xcf8594b7fcc657c893cc664a19fcfed2a489cbe1cfbb6b85e94d8e1b4bf0040`
const R3: [u64; 4] = [
    0x5e94d8e1b4bf0040,
    0x2a489cbe1cfbb6b8,
    0x893cc664a19fcfed,
    0x0cf8594b7fcc657c,
];

/// INV = -(r^{-1} mod 2^64) mod 2^64
pub const INV: u64 = 0xc2e1f593efffffff;

const S: usize = 28;

/// multiplicative group generator of n th root of unity
/// GENERATOR^t where t * 2^s + 1 = r
/// with t odd. In other words, this
/// is a 2^s root of unity.
/// `0x3ddb9f5166d18b798865ea93dd31f743215cf6dd39329c8d34f1ed960c37c9c`
pub const ROOT_OF_UNITY: Fr = Fr::to_mont_form([
    0xd34f1ed960c37c9c,
    0x3215cf6dd39329c8,
    0x98865ea93dd31f74,
    0x03ddb9f5166d18b7,
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
            0xcdcb848a1f0fac9f,
            0x0c0ac2e9419f4243,
            0x098d014dc2822db4,
            0x0000000183227397,
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

use crate::fq::Fq;
use crate::fqn::Fq2;

impl From<Fq> for Fr {
    fn from(_: Fq) -> Fr {
        unimplemented!()
    }
}

impl From<Fq2> for Fr {
    fn from(_: Fq2) -> Fr {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

    field_test!(bn254_scalar, Fr, 1000);

    #[test]
    fn test_root_of_unity() {
        let s = Fr::S;
        let mut root_of_unity = Fr::ROOT_OF_UNITY;
        (0..s).for_each(|_| root_of_unity.square_assign());
        assert_eq!(root_of_unity, Fr::one())
    }

    #[test]
    fn test_sqrt() {
        for _ in 0..100 {
            let a = Fr::random(OsRng);
            let mut b = a;
            b = b.square();

            let b = b.sqrt().unwrap();
            let mut negb = b;
            negb = negb.neg();

            assert!(a == b || a == negb);
        }
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
