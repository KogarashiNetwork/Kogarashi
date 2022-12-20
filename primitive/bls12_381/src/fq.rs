use rand_core::RngCore;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::dress::field::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
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

pairing_field_operation!(Fq, MODULUS, GENERATOR, INV, R, R2, R3);

impl Fq {
    pub(crate) const fn to_mont_form(val: [u64; 6]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }
}
