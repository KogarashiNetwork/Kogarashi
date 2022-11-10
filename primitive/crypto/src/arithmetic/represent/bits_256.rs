use crate::arithmetic::limbs::bits_256::*;
use crate::arithmetic::represent::utils::Bits;
use rand_core::RngCore;

pub const fn zero() -> [u64; 4] {
    [0, 0, 0, 0]
}

pub const fn one(r2: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    to_mont_form([1, 0, 0, 0], r2, p, inv)
}

pub const fn from_u64(val: u64) -> [u64; 4] {
    [val, 0, 0, 0]
}

pub const fn from_u512(
    limbs: [u64; 8],
    r2: [u64; 4],
    r3: [u64; 4],
    p: [u64; 4],
    inv: u64,
) -> [u64; 4] {
    let a = mul([limbs[0], limbs[1], limbs[2], limbs[3]], r2, p, inv);
    let b = mul([limbs[4], limbs[5], limbs[6], limbs[7]], r3, p, inv);
    add(a, b, p)
}

pub const fn to_mont_form(val: [u64; 4], r2: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    mul(val, r2, p, inv)
}

pub fn to_bits(val: [u64; 4]) -> Bits {
    let mut index = 256;
    let mut bits: [u8; 256] = [0; 256];
    for mut x in val {
        for _ in 0..64 {
            index -= 1;
            bits[index] = (x & 1) as u8;
            x >>= 1;
        }
    }
    bits.to_vec()
}

pub fn random_limbs(
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
