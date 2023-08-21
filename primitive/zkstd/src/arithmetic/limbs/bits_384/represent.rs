use crate::arithmetic::bits_384::*;
use crate::arithmetic::utils::{Bits, Naf, Nafs};
use crate::common::Vec;
use rand_core::RngCore;

pub const fn zero() -> [u64; 6] {
    [0; 6]
}

pub const fn one(r2: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    to_mont_form([1, 0, 0, 0, 0, 0], r2, p, inv)
}

pub const fn from_u64(val: u64, r2: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    to_mont_form([val, 0, 0, 0, 0, 0], r2, p, inv)
}

pub const fn from_u512(
    limbs: [u64; 12],
    r2: [u64; 6],
    r3: [u64; 6],
    p: [u64; 6],
    inv: u64,
) -> [u64; 6] {
    let a = mul(
        [limbs[0], limbs[1], limbs[2], limbs[3], limbs[4], limbs[5]],
        r2,
        p,
        inv,
    );
    let b = mul(
        [limbs[6], limbs[7], limbs[8], limbs[9], limbs[10], limbs[11]],
        r3,
        p,
        inv,
    );
    add(a, b, p)
}

pub const fn to_mont_form(val: [u64; 6], r2: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    mul(val, r2, p, inv)
}

#[inline(always)]
pub fn to_bits(val: [u64; 6]) -> Bits {
    let mut index = 384;
    let mut bits: [u8; 384] = [0; 384];
    for limb in val {
        for byte in limb.to_le_bytes().iter() {
            for i in 0..8 {
                index -= 1;
                bits[index] = byte >> i & 1;
            }
        }
    }
    bits.to_vec()
}

#[inline(always)]
pub fn to_nafs(val: [u64; 6]) -> Nafs {
    let mut index = 0;
    let mut bits: [u8; 386] = [0; 386];
    for limb in val {
        for byte in limb.to_le_bytes().iter() {
            for i in 0..8 {
                bits[index] = byte >> i & 1;
                index += 1;
            }
        }
    }
    let mut carry = 0;
    let mut nafs: Vec<Naf> = bits
        .iter()
        .map(|bit| {
            let triple = bit * 3;
            let bit_3 = (triple + carry) % 2;
            carry = (triple + carry) / 2;
            (bit_3 as i8 - *bit as i8).into()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip_while(|x| x == &Naf::Zero)
        .collect::<Vec<_>>();
    nafs.pop();
    nafs.reverse();
    nafs
}

pub fn random_limbs(
    mut rand: impl RngCore,
    r2: [u64; 6],
    r3: [u64; 6],
    p: [u64; 6],
    inv: u64,
) -> [u64; 6] {
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

pub const fn little_fermat(p: [u64; 6]) -> [u64; 6] {
    sub(zero(), [2, 0, 0, 0, 0, 0], p)
}
