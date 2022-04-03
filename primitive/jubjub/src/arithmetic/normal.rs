use crate::arithmetic::utils::{adc, sbb};
use crate::fr::MODULUS;

use super::utils::mac;
const INV: u64 = 0x1ba3a358ef788ef9;

pub(crate) fn add(a: &mut [u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut c = 0;
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a = adc(*a, *b, &mut c)
    }
    *a
}

pub(crate) fn sub(a: &mut [u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut c = 0;
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a = sbb(*a, *b, &mut c)
    }
    *a
}

pub(crate) fn double(a: &mut [u64; 4]) -> [u64; 4] {
    let mut c = 0;
    for a in a.iter_mut() {
        *a = adc(*a, *a, &mut c)
    }
    *a
}

pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut d = 0;
    let r0 = mac(0, a[0], b[0], &mut d);
    let r1 = mac(0, a[0], b[1], &mut d);
    let r2 = mac(0, a[0], b[2], &mut d);
    let r3 = mac(0, a[0], b[3], &mut d);
    let r4 = mac(0, a[0], b[4], &mut d);
    let r5 = mac(0, a[0], b[5], &mut d);
    let r6 = d;
    let mut d = 0;
    let r1 = mac(r1, a[1], b[0], &mut d);
    let r2 = mac(r2, a[1], b[1], &mut d);
    let r3 = mac(r3, a[1], b[2], &mut d);
    let r4 = mac(r4, a[1], b[3], &mut d);
    let r5 = mac(r5, a[1], b[4], &mut d);
    let r6 = mac(r6, a[1], b[5], &mut d);
    let r7 = d;
    let mut d = 0;
    let r2 = mac(r2, a[2], b[0], &mut d);
    let r3 = mac(r3, a[2], b[1], &mut d);
    let r4 = mac(r4, a[2], b[2], &mut d);
    let r5 = mac(r5, a[2], b[3], &mut d);
    let r6 = mac(r6, a[2], b[4], &mut d);
    let r7 = mac(r7, a[2], b[5], &mut d);
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
