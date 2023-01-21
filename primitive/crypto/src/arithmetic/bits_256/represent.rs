use crate::arithmetic::bits_256::limbs::*;
use crate::arithmetic::utils::{Bits, Naf, Nafs};
use crate::common::Vec;
use rand_core::RngCore;

pub const fn zero() -> [u64; 4] {
    [0; 4]
}

pub const fn one(r2: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    to_mont_form([1, 0, 0, 0], r2, p, inv)
}

pub const fn from_u64(val: u64, r2: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    to_mont_form([val, 0, 0, 0], r2, p, inv)
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
    for limb in val {
        for byte in limb.to_le_bytes().iter() {
            for i in 0..8 {
                index -= 1;
                bits[index] = (byte >> i & 1) as u8;
            }
        }
    }
    bits.to_vec()
}

pub fn to_nafs(val: [u64; 4]) -> Nafs {
    let mut index = 260;
    let mut bits: [u8; 260] = [0; 260];
    for limb in val {
        for byte in limb.to_le_bytes().iter() {
            for i in 0..8 {
                index -= 1;
                bits[index] = (byte >> i & 1) as u8;
            }
        }
    }
    let mut carry = 0;
    let bits_3: Vec<u8> = bits
        .iter()
        .rev()
        .map(|single| {
            let triple = single * 3;
            let res = (triple + carry) % 2;
            carry = (triple + carry) / 2;
            res
        })
        .collect();
    let nafs: Vec<Naf> = bits
        .iter()
        .rev()
        .skip(1)
        .zip(bits_3.iter().skip(1))
        .map(|(bit, bit_3)| {
            let naf = *bit_3 as i8 - *bit as i8;
            naf.into()
        })
        .collect();
    nafs
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

pub const fn little_fermat(p: [u64; 4]) -> [u64; 4] {
    sub(zero(), [2, 0, 0, 0], p)
}
