use rand_core::RngCore;
use zkstd::arithmetic::bits_384::*;
use zkstd::common::*;
use zkstd::dress::field::*;

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

#[derive(Clone, Copy, Decode, Encode)]
pub struct Fq(pub(crate) [u64; 6]);

impl SigUtils<48> for Fq {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
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

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        // SBP-M1 review: think about proper error handling instead of `unwrap`
        let l5 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap());
        let l4 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
        let l3 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap());
        let l2 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap());
        let l1 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[32..40]).unwrap());
        let l0 = u64::from_be_bytes(<[u8; 8]>::try_from(&bytes[40..48]).unwrap());

        // Try to subtract the modulus
        let (_, borrow) = sbb(l0, MODULUS[0], 0);
        let (_, borrow) = sbb(l1, MODULUS[1], borrow);
        let (_, borrow) = sbb(l2, MODULUS[2], borrow);
        let (_, borrow) = sbb(l3, MODULUS[3], borrow);
        let (_, borrow) = sbb(l4, MODULUS[4], borrow);
        let (_, borrow) = sbb(l5, MODULUS[5], borrow);

        if borrow & 1 == 1 {
            Some(Self([l5, l4, l3, l2, l1, l0]) * Self(R2))
        } else {
            None
        }
    }
}

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

    pub const fn internal_repr(&self) -> &[u64; 6] {
        &self.0
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
    pub fn sqrt(&self) -> Option<Self> {
        let sqrt = self.pow_vartime(&[
            0xee7fbfffffffeaab,
            0x7aaffffac54ffff,
            0xd9cc34a83dac3d89,
            0xd91dd2e13ce144af,
            0x92c6e9ed90d2eb35,
            0x680447a8e5ff9a6,
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

        (borrow & 1) == 0
    }
}

prime_field_operation!(Fq, MODULUS, GENERATOR, INV, R, R2, R3);
