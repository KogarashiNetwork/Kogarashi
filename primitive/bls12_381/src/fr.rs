use core::borrow::Borrow;
use core::iter::{Product, Sum};
use dusk_bytes::{Error as BytesError, Serializable};
use serde::{Deserialize, Serialize};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::arithmetic::utils::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fr(pub [u64; 4]);

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

/// Generator of the Scalar field
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

pub const ROOT_OF_UNITY: Fr = Fr([
    0xb9b58d8c5f0e466a,
    0x5b1b4c801819d7ec,
    0x0af53ae352a31e64,
    0x5bf3adda19e9b27b,
]);

impl Fr {
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
}

// below here, the crate uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and
// [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by
// Dusk-Network team and, @str4d and @ebfull

/// Two adacity
pub const TWO_ADACITY: u32 = 32;

impl Fr {
    pub fn divn(&mut self, mut n: u32) {
        if n >= 256 {
            *self = Self::from(0_u64);
            return;
        }

        while n >= 64 {
            let mut t = 0;
            for i in self.0.iter_mut().rev() {
                core::mem::swap(&mut t, i);
            }
            n -= 64;
        }

        if n > 0 {
            let mut t = 0;
            for i in self.0.iter_mut().rev() {
                let t2 = *i << (64 - n);
                *i >>= n;
                *i |= t;
                t = t2;
            }
        }
    }

    /// Exponentiates `self` by `by`, where `by` is a
    /// little-endian order integer exponent.
    ///
    /// **This operation is variable time with respect
    /// to the exponent.** If the exponent is fixed,
    /// this operation is effectively constant time.
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

    /// Computes the square root of this element, if it exists.
    pub fn sqrt(&self) -> CtOption<Self> {
        // Because r = 3 (mod 4)
        // sqrt can be done with only one exponentiation,
        // via the computation of  self^((r + 1) // 4) (mod r)
        let sqrt = self.pow_vartime(&[
            0xb425c397b5bdcb2e,
            0x299a0824f3320420,
            0x4199cec0404d0ec0,
            0x039f6d3a994cebea,
        ]);

        CtOption::new(
            sqrt,
            (sqrt * sqrt).ct_eq(self), /* Only return Some if it's the
                                        * square root. */
        )
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

impl Serializable<32> for Fr {
    type Error = BytesError;

    /// Converts an element of `Fr` into a byte representation in
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
    /// a scalar into a `Fr`, failing if the input is not canonical.
    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let mut s = [0u64; 4];

        s.iter_mut()
            .zip(buf.chunks_exact(8))
            .try_for_each(|(s, b)| {
                <[u8; 8]>::try_from(b)
                    .map(|b| *s = u64::from_le_bytes(b))
                    .map_err(|_| BytesError::InvalidData)
            })?;

        // Try to subtract the modulus
        let (_, borrow) = sbb(s[0], MODULUS[0], 0);
        let (_, borrow) = sbb(s[1], MODULUS[1], borrow);
        let (_, borrow) = sbb(s[2], MODULUS[2], borrow);
        let (_, borrow) = sbb(s[3], MODULUS[3], borrow);

        // If the element is smaller than MODULUS then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        if (borrow as u8) & 1 != 1 {
            return Err(BytesError::InvalidData);
        }

        let mut s = Fr(s);

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        s *= Self(R2);

        Ok(s)
    }
}

impl ConstantTimeEq for Fr {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
