use rand_core::RngCore;

pub mod field {
    use super::*;
    use zero_crypto::arithmetic::bits_256::*;

    pub const MODULUS: [u64; 4] = [
        0xd0970e5ed6f72cb7,
        0xa6682093ccc81082,
        0x06673b0101343b00,
        0x0e7db4ea6533afa9,
    ];

    pub const INV: u64 = 0x1ba3a358ef788ef9;

    const R2: [u64; 4] = [
        0x67719aa495e57731,
        0x51b0cef09ce3fc26,
        0x69dab7fac026e9a5,
        0x04f6547b8d127688,
    ];

    const R3: [u64; 4] = [
        0xe0d6c6563d830544,
        0x323e3883598d0f85,
        0xf0fea3004c2e2ba8,
        0x05874f84946737ec,
    ];

    pub fn random(rand: impl RngCore) -> [u64; 4] {
        random_limbs(rand, R2, R3, MODULUS, INV)
    }

    pub const fn from_raw(val: [u64; 4]) -> [u64; 4] {
        to_mont_form(val, R2, MODULUS, INV)
    }
}
