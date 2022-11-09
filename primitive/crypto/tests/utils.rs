use rand_core::RngCore;
use zero_crypto::arithmetic::limbs::bits_256::*;

pub mod jubjub_field {
    use super::*;

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
        random_logic(rand, R2, R3, MODULUS, INV)
    }

    pub fn from_raw(val: [u64; 4]) -> [u64; 4] {
        from_raw_logic(val, R2, MODULUS, INV)
    }
}

fn random_logic(
    mut rand: impl RngCore,
    r2: [u64; 4],
    r3: [u64; 4],
    p: [u64; 4],
    inv: u64,
) -> [u64; 4] {
    from_u512(
        [
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
            rand.next_u64(),
        ],
        r2,
        r3,
        p,
        inv,
    )
}

fn from_raw_logic(val: [u64; 4], r2: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    mul(val, r2, p, inv)
}

fn from_u512(limbs: [u64; 8], r2: [u64; 4], r3: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let a = mul([limbs[0], limbs[1], limbs[2], limbs[3]], r2, p, inv);
    let b = mul([limbs[4], limbs[5], limbs[6], limbs[7]], r3, p, inv);
    add(a, b, p)
}
