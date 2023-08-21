use crate::arithmetic::{bits_384::to_bits, utils::*};

#[inline(always)]
pub const fn add(a: [u64; 6], b: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, c) = addnc(a[0], b[0]);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, c) = adc(a[3], b[3], c);
    let (l4, c) = adc(a[4], b[4], c);
    let (l5, _) = adc(a[5], b[5], c);

    sub([l0, l1, l2, l3, l4, l5], p, p)
}

#[inline(always)]
pub const fn sub(a: [u64; 6], b: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, brw) = sbb(a[0], b[0], 0);
    let (l1, brw) = sbb(a[1], b[1], brw);
    let (l2, brw) = sbb(a[2], b[2], brw);
    let (l3, brw) = sbb(a[3], b[3], brw);
    let (l4, brw) = sbb(a[4], b[4], brw);
    let (l5, brw) = sbb(a[5], b[5], brw);

    let (l0, c) = addnc(l0, p[0] & brw);
    let (l1, c) = adc(l1, p[1] & brw, c);
    let (l2, c) = adc(l2, p[2] & brw, c);
    let (l3, c) = adc(l3, p[3] & brw, c);
    let (l4, c) = adc(l4, p[4] & brw, c);
    let (l5, _) = adc(l5, p[5] & brw, c);

    [l0, l1, l2, l3, l4, l5]
}

#[inline(always)]
pub const fn double(a: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    let (l0, c) = dbc(a[0], 0);
    let (l1, c) = dbc(a[1], c);
    let (l2, c) = dbc(a[2], c);
    let (l3, c) = dbc(a[3], c);
    let (l4, c) = dbc(a[4], c);
    let (l5, _) = dbc(a[5], c);

    sub([l0, l1, l2, l3, l4, l5], p, p)
}

#[inline(always)]
pub const fn mul(a: [u64; 6], b: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    let (l0, c) = mulnc(a[0], b[0]);
    let (l1, c) = muladd(a[0], b[1], c);
    let (l2, c) = muladd(a[0], b[2], c);
    let (l3, c) = muladd(a[0], b[3], c);
    let (l4, c) = muladd(a[0], b[4], c);
    let (l5, l6) = muladd(a[0], b[5], c);

    let (l1, c) = muladd(a[1], b[0], l1);
    let (l2, c) = mac(l2, a[1], b[1], c);
    let (l3, c) = mac(l3, a[1], b[2], c);
    let (l4, c) = mac(l4, a[1], b[3], c);
    let (l5, c) = mac(l5, a[1], b[4], c);
    let (l6, l7) = mac(l6, a[1], b[5], c);

    let (l2, c) = muladd(a[2], b[0], l2);
    let (l3, c) = mac(l3, a[2], b[1], c);
    let (l4, c) = mac(l4, a[2], b[2], c);
    let (l5, c) = mac(l5, a[2], b[3], c);
    let (l6, c) = mac(l6, a[2], b[4], c);
    let (l7, l8) = mac(l7, a[2], b[5], c);

    let (l3, c) = muladd(a[3], b[0], l3);
    let (l4, c) = mac(l4, a[3], b[1], c);
    let (l5, c) = mac(l5, a[3], b[2], c);
    let (l6, c) = mac(l6, a[3], b[3], c);
    let (l7, c) = mac(l7, a[3], b[4], c);
    let (l8, l9) = mac(l8, a[3], b[5], c);

    let (l4, c) = muladd(a[4], b[0], l4);
    let (l5, c) = mac(l5, a[4], b[1], c);
    let (l6, c) = mac(l6, a[4], b[2], c);
    let (l7, c) = mac(l7, a[4], b[3], c);
    let (l8, c) = mac(l8, a[4], b[4], c);
    let (l9, l10) = mac(l9, a[4], b[5], c);

    let (l5, c) = muladd(a[5], b[0], l5);
    let (l6, c) = mac(l6, a[5], b[1], c);
    let (l7, c) = mac(l7, a[5], b[2], c);
    let (l8, c) = mac(l8, a[5], b[3], c);
    let (l9, c) = mac(l9, a[5], b[4], c);
    let (l10, l11) = mac(l10, a[5], b[5], c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11], p, inv)
}

#[inline(always)]
pub const fn square(a: [u64; 6], p: [u64; 6], inv: u64) -> [u64; 6] {
    let (l1, c) = mulnc(a[1], a[0]);
    let (l2, c) = muladd(a[2], a[0], c);
    let (l3, c) = muladd(a[3], a[0], c);
    let (l4, c) = muladd(a[4], a[0], c);
    let (l5, c) = muladd(a[5], a[0], c);
    let (l6, c) = muladd(a[1], a[5], c);
    let (l7, c) = muladd(a[2], a[5], c);
    let (l8, c) = muladd(a[3], a[5], c);
    let (l9, l10) = muladd(a[4], a[5], c);

    let (l3, c) = muladd(a[1], a[2], l3);
    let (l4, c) = mac(l4, a[1], a[3], c);
    let (l5, c) = mac(l5, a[1], a[4], c);
    let (l6, c) = mac(l6, a[2], a[4], c);
    let (l7, c) = mac(l7, a[3], a[4], c);
    let (l8, c) = addnc(l8, c);
    let (l9, _) = addnc(l9, c);

    let (l5, c) = muladd(a[2], a[3], l5);
    let (l6, c) = addnc(l6, c);
    let (l7, _) = addnc(l7, c);

    let (l1, c) = dbc(l1, 0);
    let (l2, c) = dbc(l2, c);
    let (l3, c) = dbc(l3, c);
    let (l4, c) = dbc(l4, c);
    let (l5, c) = dbc(l5, c);
    let (l6, c) = dbc(l6, c);
    let (l7, c) = dbc(l7, c);
    let (l8, c) = dbc(l8, c);
    let (l9, c) = dbc(l9, c);
    let (l10, l11) = dbc(l10, c);

    let (l0, c) = mulnc(a[0], a[0]);
    let (l1, c) = addnc(l1, c);
    let (l2, c) = mac(l2, a[1], a[1], c);
    let (l3, c) = addnc(l3, c);
    let (l4, c) = mac(l4, a[2], a[2], c);
    let (l5, c) = addnc(l5, c);
    let (l6, c) = mac(l6, a[3], a[3], c);
    let (l7, c) = addnc(l7, c);
    let (l8, c) = mac(l8, a[4], a[4], c);
    let (l9, c) = addnc(l9, c);
    let (l10, c) = mac(l10, a[5], a[5], c);
    let (l11, _) = addnc(l11, c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11], p, inv)
}

#[inline(always)]
pub const fn neg(a: [u64; 6], p: [u64; 6]) -> [u64; 6] {
    if (a[0] | a[1] | a[2] | a[3] | a[4] | a[5]) == 0 {
        a
    } else {
        sub(p, a, p)
    }
}

#[inline(always)]
pub const fn mont(a: [u64; 12], p: [u64; 6], inv: u64) -> [u64; 6] {
    let rhs = a[0].wrapping_mul(inv);

    let (_, d) = muladd(rhs, p[0], a[0]);
    let (l1, d) = mac(a[1], rhs, p[1], d);
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, d) = mac(a[4], rhs, p[4], d);
    let (l5, d) = mac(a[5], rhs, p[5], d);
    let (l6, e) = addnc(a[6], d);

    let rhs = l1.wrapping_mul(inv);

    let (_, d) = muladd(rhs, p[0], l1);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, d) = mac(l5, rhs, p[4], d);
    let (l6, d) = mac(l6, rhs, p[5], d);
    let (l7, e) = adc(a[7], e, d);

    let rhs = l2.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l2);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, d) = mac(l6, rhs, p[4], d);
    let (l7, d) = mac(l7, rhs, p[5], d);
    let (l8, e) = adc(a[8], e, d);

    let rhs = l3.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l3);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, d) = mac(l7, rhs, p[4], d);
    let (l8, d) = mac(l8, rhs, p[5], d);
    let (l9, e) = adc(a[9], e, d);

    let rhs = l4.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l4);
    let (l5, d) = mac(l5, rhs, p[1], d);
    let (l6, d) = mac(l6, rhs, p[2], d);
    let (l7, d) = mac(l7, rhs, p[3], d);
    let (l8, d) = mac(l8, rhs, p[4], d);
    let (l9, d) = mac(l9, rhs, p[5], d);
    let (l10, e) = adc(a[10], e, d);

    let rhs = l5.wrapping_mul(inv);
    let (_, d) = muladd(rhs, p[0], l5);
    let (l6, d) = mac(l6, rhs, p[1], d);
    let (l7, d) = mac(l7, rhs, p[2], d);
    let (l8, d) = mac(l8, rhs, p[3], d);
    let (l9, d) = mac(l9, rhs, p[4], d);
    let (l10, d) = mac(l10, rhs, p[5], d);
    let (l11, _) = adc(a[11], e, d);

    sub([l6, l7, l8, l9, l10, l11], p, p)
}

#[inline(always)]
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

#[inline(always)]
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
