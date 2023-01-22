use crate::arithmetic::{bits_384::to_bits, utils::*};

#[inline]
pub const fn add(a: [u64; 6], b: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, c) = adc(a[3], b[3], c);
    let (l4, c) = adc(a[4], b[4], c);
    let (l5, _) = adc(a[5], b[5], c);

    sub([l0, l1, l2, l3, l4, l5], p, p)
}

#[inline]
pub const fn sub(a: [u64; 6], b: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, brw) = sbb(a[0], b[0], 0);
    let (l1, brw) = sbb(a[1], b[1], brw);
    let (l2, brw) = sbb(a[2], b[2], brw);
    let (l3, brw) = sbb(a[3], b[3], brw);
    let (l4, brw) = sbb(a[4], b[4], brw);
    let (l5, brw) = sbb(a[5], b[5], brw);

    let (l0, c) = adc(l0, p[0] & brw, 0);
    let (l1, c) = adc(l1, p[1] & brw, c);
    let (l2, c) = adc(l2, p[2] & brw, c);
    let (l3, c) = adc(l3, p[3] & brw, c);
    let (l4, c) = adc(l4, p[4] & brw, c);
    let (l5, _) = adc(l5, p[5] & brw, c);

    [l0, l1, l2, l3, l4, l5]
}

#[inline]
pub const fn double(a: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, c) = dbc(a[0], 0);
    let (l1, c) = dbc(a[1], c);
    let (l2, c) = dbc(a[2], c);
    let (l3, c) = dbc(a[3], c);
    let (l4, c) = dbc(a[4], c);
    let (l5, _) = dbc(a[5], c);

    sub([l0, l1, l2, l3, l4, l5], p, p)
}

#[inline]
pub const fn mul(a: [u64; 6], b: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    let (l0, c) = mac(0, a[0], b[0], 0);
    let (l1, c) = mac(0, a[0], b[1], c);
    let (l2, c) = mac(0, a[0], b[2], c);
    let (l3, c) = mac(0, a[0], b[3], c);
    let (l4, c) = mac(0, a[0], b[4], c);
    let (l5, l6) = mac(0, a[0], b[5], c);

    let (l1, c) = mac(l1, a[1], b[0], 0);
    let (l2, c) = mac(l2, a[1], b[1], c);
    let (l3, c) = mac(l3, a[1], b[2], c);
    let (l4, c) = mac(l4, a[1], b[3], c);
    let (l5, c) = mac(l5, a[1], b[4], c);
    let (l6, l7) = mac(l6, a[1], b[5], c);

    let (l2, c) = mac(l2, a[2], b[0], 0);
    let (l3, c) = mac(l3, a[2], b[1], c);
    let (l4, c) = mac(l4, a[2], b[2], c);
    let (l5, c) = mac(l5, a[2], b[3], c);
    let (l6, c) = mac(l6, a[2], b[4], c);
    let (l7, l8) = mac(l7, a[2], b[5], c);

    let (l3, c) = mac(l3, a[3], b[0], 0);
    let (l4, c) = mac(l4, a[3], b[1], c);
    let (l5, c) = mac(l5, a[3], b[2], c);
    let (l6, c) = mac(l6, a[3], b[3], c);
    let (l7, c) = mac(l7, a[3], b[4], c);
    let (l8, l9) = mac(l8, a[3], b[5], c);

    let (l4, c) = mac(l4, a[4], b[0], 0);
    let (l5, c) = mac(l5, a[4], b[1], c);
    let (l6, c) = mac(l6, a[4], b[2], c);
    let (l7, c) = mac(l7, a[4], b[3], c);
    let (l8, c) = mac(l8, a[4], b[4], c);
    let (l9, l10) = mac(l9, a[4], b[5], c);

    let (l5, c) = mac(l5, a[5], b[0], 0);
    let (l6, c) = mac(l6, a[5], b[1], c);
    let (l7, c) = mac(l7, a[5], b[2], c);
    let (l8, c) = mac(l8, a[5], b[3], c);
    let (l9, c) = mac(l9, a[5], b[4], c);
    let (l10, l11) = mac(l10, a[5], b[5], c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11], p, inv)
}

#[inline]
pub const fn square(a: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    let a10 = a[1] as u128 * a[0] as u128;
    let a20 = a[2] as u128 * a[0] as u128;
    let a30 = a[3] as u128 * a[0] as u128;
    let a40 = a[4] as u128 * a[0] as u128;
    let a50 = a[5] as u128 * a[0] as u128;
    let a12 = a[1] as u128 * a[2] as u128;
    let a13 = a[1] as u128 * a[3] as u128;
    let a14 = a[1] as u128 * a[4] as u128;
    let a15 = a[1] as u128 * a[5] as u128;
    let a23 = a[2] as u128 * a[3] as u128;
    let a24 = a[2] as u128 * a[4] as u128;
    let a25 = a[2] as u128 * a[5] as u128;
    let a34 = a[3] as u128 * a[4] as u128;
    let a35 = a[3] as u128 * a[5] as u128;
    let a45 = a[4] as u128 * a[5] as u128;

    let (l0, c) = mulnc(a[0], a[0]);
    let (l1, c) = addnc(a10, c);
    let (l2, c) = addnc(a20, c);
    let (l3, c) = addnc(a30, c);
    let (l4, c) = addnc(a40, c);
    let (l5, l6) = addnc(a50, c);

    let (l1, c) = addnc(a10, l1);
    let (l2, c) = macnc(l2, a20, c);
    let (l3, c) = macnc(l3, a30, c);
    let (l4, c) = macnc(l4, a40, c);
    let (l5, c) = macnc(l5, a50, c);
    let (l6, l7) = macnc(l6, a15, c);

    let (l2, c) = mac(l2, a[1], a[1], 0);
    let (l3, c) = macnc(l3, a12, c);
    let (l4, c) = macnc(l4, a13, c);
    let (l5, c) = macnc(l5, a14, c);
    let (l6, c) = macnc(l6, a15, c);
    let (l7, l8) = macnc(l7, a25, c);

    let (l3, c) = addnc(a12, l3);
    let (l4, c) = macnc(l4, a13, c);
    let (l5, c) = macnc(l5, a14, c);
    let (l6, c) = macnc(l6, a24, c);
    let (l7, c) = macnc(l7, a25, c);
    let (l8, l9) = macnc(l8, a35, c);

    let (l4, c) = mac(l4, a[2], a[2], 0);
    let (l5, c) = macnc(l5, a23, c);
    let (l6, c) = macnc(l6, a24, c);
    let (l7, c) = macnc(l7, a34, c);
    let (l8, c) = macnc(l8, a35, c);
    let (l9, l10) = macnc(l9, a45, c);

    let (l5, c) = addnc(a23, l5);
    let (l6, c) = mac(l6, a[3], a[3], c);
    let (l7, c) = macnc(l7, a34, c);
    let (l8, c) = mac(l8, a[4], a[4], c);
    let (l9, c) = macnc(l9, a45, c);
    let (l10, l11) = mac(l10, a[5], a[5], c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11], p, inv)
}

#[inline]
pub const fn neg(a: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    if (a[0] | a[1] | a[2] | a[3] | a[4] | a[5]) == 0 {
        a
    } else {
        sub(p, a, p)
    }
}

#[inline]
pub const fn mont(a: [u64; 12], p: [u64; 6], inv: u64) -> [u64; 6] {
    let rhs = a[0].wrapping_mul(inv);

    let (_, d) = mac(a[0], rhs, p[0], 0);
    let (l1, d) = mac(a[1], rhs, p[1], d);
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, d) = mac(a[4], rhs, p[4], d);
    let (l5, d) = mac(a[5], rhs, p[5], d);
    let (l6, e) = adc(a[6], 0, d);

    let rhs = l1.wrapping_mul(inv);

    let (_, d) = mac(l1, rhs, p[0], 0);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, d) = mac(l5, rhs, p[4], d);
    let (l6, d) = mac(l6, rhs, p[5], d);
    let (l7, e) = adc(a[7], e, d);

    let rhs = l2.wrapping_mul(inv);
    let (_, d) = mac(l2, rhs, p[0], 0);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, d) = mac(l6, rhs, p[4], d);
    let (l7, d) = mac(l7, rhs, p[5], d);
    let (l8, e) = adc(a[8], e, d);

    let rhs = l3.wrapping_mul(inv);
    let (_, d) = mac(l3, rhs, p[0], 0);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, d) = mac(l7, rhs, p[4], d);
    let (l8, d) = mac(l8, rhs, p[5], d);
    let (l9, e) = adc(a[9], e, d);

    let rhs = l4.wrapping_mul(inv);
    let (_, d) = mac(l4, rhs, p[0], 0);
    let (l5, d) = mac(l5, rhs, p[1], d);
    let (l6, d) = mac(l6, rhs, p[2], d);
    let (l7, d) = mac(l7, rhs, p[3], d);
    let (l8, d) = mac(l8, rhs, p[4], d);
    let (l9, d) = mac(l9, rhs, p[5], d);
    let (l10, e) = adc(a[10], e, d);

    let rhs = l5.wrapping_mul(inv);
    let (_, d) = mac(l5, rhs, p[0], 0);
    let (l6, d) = mac(l6, rhs, p[1], d);
    let (l7, d) = mac(l7, rhs, p[2], d);
    let (l8, d) = mac(l8, rhs, p[3], d);
    let (l9, d) = mac(l9, rhs, p[4], d);
    let (l10, d) = mac(l10, rhs, p[5], d);
    let (l11, _) = adc(a[11], e, d);

    sub([l6, l7, l8, l9, l10, l11], p, p)
}

#[inline]
pub fn invert(
    a: [u64; 6],
    little_fermat: [u64; 6],
    identity: [u64; 6],
    p: [u64; 6],
    inv: u64,
) -> Option<[u64; 6]> {
    let zero: [u64; 6] = [0, 0, 0, 0, 0, 0];
    if a == zero {
        None
    } else {
        Some(pow(a, little_fermat, identity, p, inv))
    }
}

pub fn pow(a: [u64; 6], b: [u64; 6], mut identity: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    let zero: [u64; 6] = [0; 6];
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
