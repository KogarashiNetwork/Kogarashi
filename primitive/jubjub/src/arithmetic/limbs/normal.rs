use super::utils::{adc, mac, rdc, sbb};
const INV: u64 = 0x1ba3a358ef788ef9;

pub(crate) const fn add(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);
    reduce(&[l0, l1, l2, l3], p)
}

pub(crate) const fn reduce(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = rdc(a[0], b[0], 0);
    let (l1, c) = rdc(a[1], b[1], c);
    let (l2, c) = rdc(a[2], b[2], c);
    let (l3, c) = rdc(a[3], b[3], c);

    if c == 1 {
        *a
    } else {
        [l0, l1, l2, l3]
    }
}

pub(crate) const fn sub(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = sbb(a[0], b[0], 0);
    let (l1, c) = sbb(a[1], b[1], c);
    let (l2, c) = sbb(a[2], b[2], c);
    let (l3, c) = sbb(a[3], b[3], c);
    let res = [l0, l1, l2, l3];
    if c == 1 {
        add(&res, p, p)
    } else {
        res
    }
}

pub(crate) const fn double(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], a[0], 0);
    let (l1, c) = adc(a[1], a[1], c);
    let (l2, c) = adc(a[2], a[2], c);
    let (l3, _) = adc(a[3], a[3], c);
    reduce(&[l0, l1, l2, l3], p)
}

pub(crate) const fn mul(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (r0, d) = mac(0, a[0], b[0], 0);
    let (r1, d) = mac(0, a[0], b[1], d);
    let (r2, d) = mac(0, a[0], b[2], d);
    let (r3, d) = mac(0, a[0], b[3], d);
    let r4 = d;
    let (r1, d) = mac(r1, a[1], b[0], 0);
    let (r2, d) = mac(r2, a[1], b[1], d);
    let (r3, d) = mac(r3, a[1], b[2], d);
    let (r4, d) = mac(r4, a[1], b[3], d);
    let r5 = d;
    let (r2, d) = mac(r2, a[2], b[0], 0);
    let (r3, d) = mac(r3, a[2], b[1], d);
    let (r4, d) = mac(r4, a[2], b[2], d);
    let (r5, d) = mac(r5, a[2], b[3], d);
    let r6 = d;
    let (r3, d) = mac(r3, a[3], b[0], 0);
    let (r4, d) = mac(r4, a[3], b[1], d);
    let (r5, d) = mac(r5, a[3], b[2], d);
    let (r6, d) = mac(r6, a[3], b[3], d);
    let r7 = d;
    mont(&[r0, r1, r2, r3, r4, r5, r6, r7], p)
}

pub(crate) const fn square(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    mul(a, a, p)
}

pub(crate) fn neg(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    if a == &[0; 4] {
        *a
    } else {
        sub(p, a, p)
    }
}

pub(crate) const fn mont(a: &[u64; 8], p: &[u64; 4]) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(INV);
    let (_, d) = mac(a[0], rhs, p[0], 0);
    let (l1, d) = mac(a[1], rhs, p[1], d);
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, e) = adc(a[4], 0, d);

    let rhs = l1.wrapping_mul(INV);
    let (_, d) = mac(l1, rhs, p[0], 0);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, e) = adc(a[5], e, d);

    let rhs = l2.wrapping_mul(INV);
    let (_, d) = mac(l2, rhs, p[0], 0);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, e) = adc(a[6], e, d);

    let rhs = l3.wrapping_mul(INV);
    let (_, d) = mac(l3, rhs, p[0], 0);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, _) = adc(a[7], e, d);

    reduce(&[l4, l5, l6, l7], p)
}
