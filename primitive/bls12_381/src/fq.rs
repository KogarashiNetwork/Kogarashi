use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Clone, Copy, Decode, Encode)]
pub struct Fq(pub(crate) [u64; 6]);

const MODULUS: [u64; 6] = [
    0xb9feffffffffaaab,
    0x1eabfffeb153ffff,
    0x6730d2a0f6b0f624,
    0x64774b84f38512bf,
    0x4b1ba7b6434bacd7,
    0x1a0111ea397fe69a,
];

const GENERATOR: [u64; 6] = [2, 0, 0, 0, 0, 0];

/// R = 2^384 mod p
const R: [u64; 6] = [
    0x760900000002fffd,
    0xebf4000bc40c0002,
    0x5f48985753c758ba,
    0x77ce585370525745,
    0x5c071a97a256ec6d,
    0x15f65ec3fa80e493,
];

/// R2 = 2^(384*2) mod p
const R2: [u64; 6] = [
    0xf4df1f341c341746,
    0x0a76e6a609d104f1,
    0x8de5476c4c95b6d5,
    0x67eb88a9939d83c0,
    0x9a793e85b519952d,
    0x11988fe592cae3aa,
];

/// R3 = 2^(384*3) mod p
const R3: [u64; 6] = [
    0xed48ac6bd94ca1e0,
    0x315f831e03a7adf8,
    0x9a53352a615e29dd,
    0x34c04e5e921e1761,
    0x2512d43565724728,
    0x0aa6346091755d4d,
];

const INV: u64 = 0x89f3fffcfffcfffd;

impl Fq {
    pub(crate) const fn to_mont_form(val: [u64; 6]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }

    pub(crate) const fn montgomery_reduce(self) -> [u64; 6] {
        mont(
            [
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], 0, 0, 0, 0, 0, 0,
            ],
            MODULUS,
            INV,
        )
    }
}

prime_field_operation!(Fq, MODULUS, GENERATOR, INV, R, R2, R3);

// below here, the crate uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and
// [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by
// Dusk-Network team and, @str4d and @ebfull

impl Fq {
    /// Internal representation of `Fp`
    pub const fn internal_repr(&self) -> &[u64; 6] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; 48] {
        let tmp = self.montgomery_reduce();

        let mut res = [0; 48];
        res[0..8].copy_from_slice(&tmp[5].to_be_bytes());
        res[8..16].copy_from_slice(&tmp[4].to_be_bytes());
        res[16..24].copy_from_slice(&tmp[3].to_be_bytes());
        res[24..32].copy_from_slice(&tmp[2].to_be_bytes());
        res[32..40].copy_from_slice(&tmp[1].to_be_bytes());
        res[40..48].copy_from_slice(&tmp[0].to_be_bytes());

        res
    }

    pub fn lexicographically_largest(&self) -> Choice {
        // This can be determined by checking to see if the element is
        // larger than (p - 1) // 2. If we subtract by ((p - 1) // 2) + 1
        // and there is no underflow, then the element must be larger than
        // (p - 1) // 2.

        // First, because self is in Montgomery form we need to reduce it
        let tmp = self.montgomery_reduce();

        let (_, borrow) = sbb(tmp[0], 0xdcff7fffffffd556, 0);
        let (_, borrow) = sbb(tmp[1], 0x0f55ffff58a9ffff, borrow);
        let (_, borrow) = sbb(tmp[2], 0xb39869507b587b12, borrow);
        let (_, borrow) = sbb(tmp[3], 0xb23ba5c279c2895f, borrow);
        let (_, borrow) = sbb(tmp[4], 0x258dd3db21a5d66b, borrow);
        let (_, borrow) = sbb(tmp[5], 0x0d0088f51cbff34d, borrow);

        // If the element was smaller, the subtraction will underflow
        // producing a borrow value of 0xffff...ffff, otherwise it will
        // be zero. We create a Choice representing true if there was
        // overflow (and so this element is not lexicographically larger
        // than its negation) and then negate it.

        !Choice::from((borrow as u8) & 1)
    }

    pub fn from_bytes(bytes: &[u8; 48]) -> CtOption<Self> {
        let mut tmp = Self([0, 0, 0, 0, 0, 0]);

        tmp.0[5] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap());
        tmp.0[4] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
        tmp.0[3] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap());
        tmp.0[2] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap());
        tmp.0[1] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[32..40]).unwrap());
        tmp.0[0] = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[40..48]).unwrap());

        // Try to subtract the modulus
        let (_, borrow) = sbb(tmp.0[0], MODULUS[0], 0);
        let (_, borrow) = sbb(tmp.0[1], MODULUS[1], borrow);
        let (_, borrow) = sbb(tmp.0[2], MODULUS[2], borrow);
        let (_, borrow) = sbb(tmp.0[3], MODULUS[3], borrow);
        let (_, borrow) = sbb(tmp.0[4], MODULUS[4], borrow);
        let (_, borrow) = sbb(tmp.0[5], MODULUS[5], borrow);

        // If the element is smaller than MODULUS then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp *= Self(R2);

        CtOption::new(tmp, Choice::from(is_some))
    }

    pub fn pow_vartime(&self, by: &[u64; 6]) -> Self {
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
    pub fn sqrt(&self) -> CtOption<Self> {
        // We use Shank's method, as p = 3 (mod 4). This means
        // we only need to exponentiate by (p+1)/4. This only
        // works for elements that are actually quadratic residue,
        // so we check that we got the correct result at the end.

        let sqrt = self.pow_vartime(&[
            0xee7fbfffffffeaab,
            0x7aaffffac54ffff,
            0xd9cc34a83dac3d89,
            0xd91dd2e13ce144af,
            0x92c6e9ed90d2eb35,
            0x680447a8e5ff9a6,
        ]);

        CtOption::new(sqrt, sqrt.square().ct_eq(self))
    }
}

impl ConstantTimeEq for Fq {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0[0].ct_eq(&other.0[0])
            & self.0[1].ct_eq(&other.0[1])
            & self.0[2].ct_eq(&other.0[2])
            & self.0[3].ct_eq(&other.0[3])
            & self.0[4].ct_eq(&other.0[4])
            & self.0[5].ct_eq(&other.0[5])
    }
}

impl ConditionallySelectable for Fq {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Fq([
            u64::conditional_select(&a.0[0], &b.0[0], choice),
            u64::conditional_select(&a.0[1], &b.0[1], choice),
            u64::conditional_select(&a.0[2], &b.0[2], choice),
            u64::conditional_select(&a.0[3], &b.0[3], choice),
            u64::conditional_select(&a.0[4], &b.0[4], choice),
            u64::conditional_select(&a.0[5], &b.0[5], choice),
        ])
    }
}
