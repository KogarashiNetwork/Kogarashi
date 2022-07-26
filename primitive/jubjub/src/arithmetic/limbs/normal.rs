use super::utils::{adc, mac, sbb};
const INV: u64 = 0x1ba3_a358_ef78_8ef9;

pub(crate) fn add(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);

    sub(&[l0, l1, l2, l3], p, p)
}

#[inline]
pub(crate) fn sub(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
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

pub(crate) fn double(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    add(a, a, p)
}

#[inline]
pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = mac(0, a[0], b[0], 0);
    let (l1, c) = mac(0, a[0], b[1], c);
    let (l2, c) = mac(0, a[0], b[2], c);
    let (l3, l4) = mac(0, a[0], b[3], c);

    let (l1, c) = mac(l1, a[1], b[0], 0);
    let (l2, c) = mac(l2, a[1], b[1], c);
    let (l3, c) = mac(l3, a[1], b[2], c);
    let (l4, l5) = mac(l4, a[1], b[3], c);

    let (l2, c) = mac(l2, a[2], b[0], 0);
    let (l3, c) = mac(l3, a[2], b[1], c);
    let (l4, c) = mac(l4, a[2], b[2], c);
    let (l5, l6) = mac(l5, a[2], b[3], c);

    let (l3, c) = mac(l3, a[3], b[0], 0);
    let (l4, c) = mac(l4, a[3], b[1], c);
    let (l5, c) = mac(l5, a[3], b[2], c);
    let (l6, l7) = mac(l6, a[3], b[3], c);

    mont(&[l0, l1, l2, l3, l4, l5, l6, l7], p)
}

pub(crate) fn square(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    mul(a, a, p)
}

pub(crate) fn neg(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    if a == &[0; 4] {
        *a
    } else {
        sub(p, a, p)
    }
}

pub(crate) fn mont(a: &[u64; 8], p: &[u64; 4]) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(INV);

    let (_, d) = mac(a[0], rhs, p[0], 0); // a + (b * c) + d = 4 + INV * MOD[0] + 0;
    let (l1, d) = mac(a[1], rhs, p[1], d); //
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

    sub(&[l4, l5, l6, l7], p, p)
}

#[cfg(test)]
mod test {
    use crate::fr::Fr;

    #[test]
    fn two_square() {
        let x = Fr::from_raw([2, 0, 0, 0]);
        let y = Fr::from_raw([4, 0, 0, 0]);

        assert_eq!(x.square(), y);
    }
}
