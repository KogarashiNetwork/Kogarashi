use crate::arithmetic::utils::{adc, sbb};
use crate::fr::MODULUS;

use super::utils::mac;
const INV: u64 = 0x1ba3a358ef788ef9;

pub(crate) const fn add(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);
    [l0, l1, l2, l3]
}

pub(crate) const fn sub(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = sbb(a[0], b[0], 0);
    let (l1, c) = sbb(a[1], b[1], c);
    let (l2, c) = sbb(a[2], b[2], c);
    let (l3, _) = sbb(a[3], b[3], c);
    [l0, l1, l2, l3]
}

pub(crate) const fn double(a: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], a[0], 0);
    let (l1, c) = adc(a[1], a[1], c);
    let (l2, c) = adc(a[2], a[2], c);
    let (l3, _) = adc(a[3], a[3], c);
    [l0, l1, l2, l3]
}

pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut d = 0;
    let r0 = mac(0, a[0], b[0], &mut d);
    let r1 = mac(0, a[0], b[1], &mut d);
    let r2 = mac(0, a[0], b[2], &mut d);
    let r3 = mac(0, a[0], b[3], &mut d);
    let r4 = d;
    let mut d = 0;
    let r1 = mac(r1, a[1], b[0], &mut d);
    let r2 = mac(r2, a[1], b[1], &mut d);
    let r3 = mac(r3, a[1], b[2], &mut d);
    let r4 = mac(r4, a[1], b[3], &mut d);
    let r5 = d;
    let mut d = 0;
    let r2 = mac(r2, a[2], b[0], &mut d);
    let r3 = mac(r3, a[2], b[1], &mut d);
    let r4 = mac(r4, a[2], b[2], &mut d);
    let r6 = mac(r5, a[2], b[3], &mut d);
    let r7 = d;
    mont(&mut [r0, r1, r2, r3, r4, r5, r6, r7])
}

pub(crate) fn square(a: &[u64; 4]) -> [u64; 4] {
    mul(a, a)
}

pub(crate) fn neg(a: &[u64; 4]) -> [u64; 4] {
    let mut modulus = MODULUS.clone();
    sub(&mut modulus, a)
}

pub(crate) fn mont(a: &mut [u64; 8]) -> [u64; 4] {
    let mut c = 0;
    let mut c2 = 0;

    for i in 0..4 {
        let mut offset = i;
        let b = a[i] * INV;
        a[offset] = mac(a[offset], b, MODULUS[0], &mut c);
        offset += 1;
        a[offset] = mac(a[offset], b, MODULUS[0], &mut c);
        offset += 1;
        a[offset] = mac(a[offset], b, MODULUS[0], &mut c);
        offset += 1;
        a[offset] = mac(a[offset], b, MODULUS[0], &mut c);
        c2 = c;
        c = 0;
    }

    [a[0], a[1], a[2], a[3]]
}
