use crate::arithmetic::{bits_256::*, utils::*};

#[inline]
pub const fn add(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, c) = addnc(a[0], b[0]);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);

    sub([l0, l1, l2, l3], p, p)
}

#[inline]
pub const fn sub(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, brw) = sbb(a[0], b[0], 0);
    let (l1, brw) = sbb(a[1], b[1], brw);
    let (l2, brw) = sbb(a[2], b[2], brw);
    let (l3, brw) = sbb(a[3], b[3], brw);

    let (l0, c) = adc(l0, p[0] & brw, 0);
    let (l1, c) = adc(l1, p[1] & brw, c);
    let (l2, c) = adc(l2, p[2] & brw, c);
    let (l3, _) = adc(l3, p[3] & brw, c);

    [l0, l1, l2, l3]
}

#[inline]
pub const fn double(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, c) = dbc(a[0], 0);
    let (l1, c) = dbc(a[1], c);
    let (l2, c) = dbc(a[2], c);
    let (l3, _) = dbc(a[3], c);

    sub([l0, l1, l2, l3], p, p)
}

#[inline]
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

#[inline]
pub const fn square(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (l1, c) = mulnc(a[1], a[0]);
    let (l2, c) = muladd(a[2], a[0], c);
    let (l3, c) = muladd(a[3], a[0], c);
    let (l4, c) = muladd(a[1], a[3], c);
    let (l5, l6) = muladd(a[2], a[3], c);
    let (l3, c) = muladd(a[1], a[2], l3);
    let (l4, c) = addnc(l4, c);
    let (l5, _) = addnc(l5, c);

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
    let (l7, _) = addnc(l7, c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7], p, inv)
}

#[inline]
pub const fn neg(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    if (a[0] | a[1] | a[2] | a[3]) == 0 {
        a
    } else {
        sub(p, a, p)
    }
}

#[inline]
pub const fn mont(a: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(inv);

    let (_, d) = muladd(rhs, p[0], a[0]);
    let (l1, d) = mac(a[1], rhs, p[1], d);
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, e) = addnc(a[4], d);

    let rhs = l1.wrapping_mul(inv);

    let (_, d) = muladd(rhs, p[0], l1);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, e) = adc(a[5], e, d);

    let rhs = l2.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l2);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, e) = adc(a[6], e, d);

    let rhs = l3.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l3);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, _) = adc(a[7], e, d);

    sub([l4, l5, l6, l7], p, p)
}

// 54M + 248S
#[inline]
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
