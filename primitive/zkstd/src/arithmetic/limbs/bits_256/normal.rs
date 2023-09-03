use crate::arithmetic::{bits_256::*, utils::*};

#[inline(always)]
pub const fn add(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let s = a[0] as u128 + b[0] as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = a[1] as u128 + b[1] as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = a[2] as u128 + b[2] as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = a[3].wrapping_add(b[3]).wrapping_add(c);

    let s = (l0 as u128).wrapping_sub(p[0] as u128);
    let (l0, brw) = (s as u64, (s >> 64) as u64);
    let s = (l1 as u128).wrapping_sub(p[1] as u128 + (brw >> 63) as u128);
    let (l1, brw) = (s as u64, (s >> 64) as u64);
    let s = (l2 as u128).wrapping_sub(p[2] as u128 + (brw >> 63) as u128);
    let (l2, brw) = (s as u64, (s >> 64) as u64);
    let s = (l3 as u128).wrapping_sub(p[3] as u128 + (brw >> 63) as u128);
    let (l3, brw) = (s as u64, (s >> 64) as u64);

    let s = l0 as u128 + (p[0] & brw) as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = l1 as u128 + (p[1] & brw) as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = l2 as u128 + (p[2] & brw) as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = l3.wrapping_add(p[3] & brw).wrapping_add(c);

    [l0, l1, l2, l3]
}

#[inline(always)]
pub const fn sub(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let s = (a[0] as u128).wrapping_sub(b[0] as u128);
    let (l0, brw) = (s as u64, (s >> 64) as u64);
    let s = (a[1] as u128).wrapping_sub(b[1] as u128 + (brw >> 63) as u128);
    let (l1, brw) = (s as u64, (s >> 64) as u64);
    let s = (a[2] as u128).wrapping_sub(b[2] as u128 + (brw >> 63) as u128);
    let (l2, brw) = (s as u64, (s >> 64) as u64);
    let s = (a[3] as u128).wrapping_sub(b[3] as u128 + (brw >> 63) as u128);
    let (l3, brw) = (s as u64, (s >> 64) as u64);

    let s = l0 as u128 + (p[0] & brw) as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = l1 as u128 + (p[1] & brw) as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = l2 as u128 + (p[2] & brw) as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = l3.wrapping_add(p[3] & brw).wrapping_add(c);

    [l0, l1, l2, l3]
}

#[inline(always)]
pub const fn double(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, c) = (a[0] << 1, a[0] >> 63);
    let s = ((a[1] as u128) << 1) + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = ((a[2] as u128) << 1) + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = (a[3] << 1).wrapping_add(c);

    let s = (l0 as u128).wrapping_sub(p[0] as u128);
    let (l0, brw) = (s as u64, (s >> 64) as u64);
    let s = (l1 as u128).wrapping_sub(p[1] as u128 + (brw >> 63) as u128);
    let (l1, brw) = (s as u64, (s >> 64) as u64);
    let s = (l2 as u128).wrapping_sub(p[2] as u128 + (brw >> 63) as u128);
    let (l2, brw) = (s as u64, (s >> 64) as u64);
    let s = (l3 as u128).wrapping_sub(p[3] as u128 + (brw >> 63) as u128);
    let (l3, brw) = (s as u64, (s >> 64) as u64);

    let s = l0 as u128 + (p[0] & brw) as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = l1 as u128 + (p[1] & brw) as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = l2 as u128 + (p[2] & brw) as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = l3.wrapping_add(p[3] & brw).wrapping_add(c);

    [l0, l1, l2, l3]
}

#[inline(always)]
pub const fn mul(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (l0, c) = mulnc(a[0], b[0]);
    let (l1, c) = muladd(a[0], b[1], c);
    let (l2, c) = muladd(a[0], b[2], c);
    let (l3, l4) = muladd(a[0], b[3], c);

    let (l1, c) = muladd(a[1], b[0], l1);
    let (l2, c) = mac(l2, a[1], b[1], c);
    let (l3, c) = mac(l3, a[1], b[2], c);
    let (l4, l5) = mac(l4, a[1], b[3], c);

    let (l2, c) = muladd(a[2], b[0], l2);
    let (l3, c) = mac(l3, a[2], b[1], c);
    let (l4, c) = mac(l4, a[2], b[2], c);
    let (l5, l6) = mac(l5, a[2], b[3], c);

    let (l3, c) = muladd(a[3], b[0], l3);
    let (l4, c) = mac(l4, a[3], b[1], c);
    let (l5, c) = mac(l5, a[3], b[2], c);
    let (l6, l7) = mac(l6, a[3], b[3], c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7], p, inv)
}

#[inline(always)]
pub const fn square(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (l1, c) = mulnc(a[1], a[0]);
    let (l2, c) = muladd(a[2], a[0], c);
    let (l3, c) = muladd(a[3], a[0], c);
    let (l4, c) = muladd(a[1], a[3], c);
    let (l5, l6) = muladd(a[2], a[3], c);
    let (l3, c) = muladd(a[1], a[2], l3);
    let (l4, c) = addnc(l4, c);
    let l5 = addncskip(l5, c);

    let (l1, c) = dbc(l1, 0);
    let (l2, c) = dbc(l2, c);
    let (l3, c) = dbc(l3, c);
    let (l4, c) = dbc(l4, c);
    let (l5, c) = dbc(l5, c);
    let (l6, l7) = dbc(l6, c);

    let (l0, c) = mulnc(a[0], a[0]);
    let (l1, c) = addnc(l1, c);
    let (l2, c) = mac(l2, a[1], a[1], c);
    let (l3, c) = addnc(l3, c);
    let (l4, c) = mac(l4, a[2], a[2], c);
    let (l5, c) = addnc(l5, c);
    let (l6, c) = mac(l6, a[3], a[3], c);
    let l7 = addncskip(l7, c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7], p, inv)
}

#[inline(always)]
pub const fn neg(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    if (a[0] | a[1] | a[2] | a[3]) == 0 {
        a
    } else {
        sub(p, a, p)
    }
}

#[inline(always)]
pub const fn mont(a: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(inv);

    let d = muladdbackskip(rhs, p[0], a[0]);
    let (l1, d) = mac(a[1], rhs, p[1], d);
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, e) = addnc(a[4], d);

    let rhs = l1.wrapping_mul(inv);

    let d = muladdbackskip(rhs, p[0], l1);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, e) = adc(a[5], e, d);

    let rhs = l2.wrapping_mul(inv);
    let d = muladdbackskip(rhs, p[0], l2);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, e) = adc(a[6], e, d);

    let rhs = l3.wrapping_mul(inv);
    let d = muladdbackskip(rhs, p[0], l3);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let l7 = adcskip(a[7], e, d);

    sub([l4, l5, l6, l7], p, p)
}

#[inline(always)]
pub fn invert(
    a: [u64; 4],
    little_fermat: [u64; 4],
    identity: [u64; 4],
    p: [u64; 4],
    inv: u64,
) -> Option<[u64; 4]> {
    let zero: [u64; 4] = [0; 4];
    if a == zero {
        None
    } else {
        Some(pow(a, little_fermat, identity, p, inv))
    }
}

#[inline(always)]
pub fn pow(a: [u64; 4], b: [u64; 4], mut identity: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let zero: [u64; 4] = [0; 4];
    if b == zero {
        return identity;
    } else if a == zero {
        return zero;
    }
    let bits = to_bits(b);
    for &bit in bits.iter() {
        identity = square(identity, p, inv);
        if bit == 1 {
            identity = mul(identity, a, p, inv);
        }
    }
    identity
}
