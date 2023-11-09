use crate::Fr;
use zkstd::arithmetic::bits_256::*;
use zkstd::common::*;
use zkstd::macros::field::*;

/// Constant representing the modulus
/// q = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47
const MODULUS: [u64; 4] = [
    0x3c208c16d87cfd47,
    0x97816a916871ca8d,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

const GENERATOR: [u64; 4] = [3, 0, 0, 0];

/// R = 2^256 mod q
const R: [u64; 4] = [
    0xd35d438dc58f0d9d,
    0x0a78eb28f5c70b3d,
    0x666ea36f7879462c,
    0x0e0a77c19a07df2f,
];

/// R^2 = 2^512 mod q
const R2: [u64; 4] = [
    0xf32cfc5b538afa89,
    0xb5e71911d44501fb,
    0x47ab1eff0a417ff6,
    0x06d89f71cab8351f,
];

/// R^3 = 2^768 mod q
const R3: [u64; 4] = [
    0xb1cd6dafda1530df,
    0x62f210e6a7283db6,
    0xef7f0b0c0ada0afb,
    0x20fd6e902d592544,
];

/// INV = -(q^{-1} mod 2^64) mod 2^64
const INV: u64 = 0x87d20782e4866389;

/// bn254 curve base field
#[derive(Clone, Copy, Decode, Encode)]
pub struct Fq(pub(crate) [u64; 4]);

impl SigUtils<32> for Fq {
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

impl Fq {
    pub const fn new_unchecked(val: [u64; 4]) -> Self {
        Self(val)
    }
    pub const fn add_const(self, rhs: Self) -> Self {
        Self(add(self.0, rhs.0, MODULUS))
    }

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

    pub const fn inner(&self) -> &[u64; 4] {
        &self.0
    }

    pub fn pow_vartime(&self, by: &[u64; 4]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res *= *self;
                }
            }
        }
        res
    }

    #[inline]
    pub fn sqrt(&self) -> Option<Self> {
        let sqrt = self.pow_vartime(&[
            0x4f082305b61f3f52,
            0x65e05aa45a1c72a3,
            0x6e14116da0605617,
            0x0c19139cb84c680a,
        ]);

        match sqrt.square() == *self {
            true => Some(sqrt),
            false => None,
        }
    }

    pub fn lexicographically_largest(&self) -> bool {
        // This can be determined by checking to see if the element is
        // larger than (p - 1) // 2. If we subtract by ((p - 1) // 2) + 1
        // and there is no underflow, then the element must be larger than
        // (p - 1) // 2.

        // First, because self is in Montgomery form we need to reduce it
        let tmp = self.montgomery_reduce();

        let (_, borrow) = sbb(tmp[0], 0x9e10460b6c3e7ea4, 0);
        let (_, borrow) = sbb(tmp[1], 0xcbc0b548b438e546, borrow);
        let (_, borrow) = sbb(tmp[2], 0xdc2822db40c0ac2e, borrow);
        let (_, borrow) = sbb(tmp[3], 0x183227397098d014, borrow);

        // If the element was smaller, the subtraction will underflow
        // producing a borrow value of 0xffff...ffff, otherwise it will
        // be zero. We create a Choice representing true if there was
        // overflow (and so this element is not lexicographically larger
        // than its negation) and then negate it.

        (borrow & 1) == 0
    }
}

prime_field_operation!(Fq, MODULUS, GENERATOR, INV, R, R2, R3);

impl From<Fr> for Fq {
    fn from(val: Fr) -> Fq {
        Self(val.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn test_serde() {
        for _ in 0..100000 {
            let s = Fq::random(OsRng);
            let bytes = s.to_bytes();
            let s_prime = Fq::from_bytes(bytes).unwrap();
            assert_eq!(s, s_prime);
        }
    }
}
