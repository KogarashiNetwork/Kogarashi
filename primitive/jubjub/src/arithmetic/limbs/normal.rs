use libc_print::libc_println;

use super::utils::{adc, mac, sbb};
const INV: u64 = 0x1ba3a358ef788ef9;

fn print_hex<'a>(a: impl IntoIterator<Item = &'a u64>) {
    a.into_iter().for_each(|x| {
        libc_print::libc_print!("{:x} ", x);
    });
    libc_println!();
}

pub(crate) fn add(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    libc_println!("Add");
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);

    sub(&[l0, l1, l2, l3], p, p)
}

#[inline]
pub(crate) fn sub(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    libc_println!("Sub");
    print_hex(a);
    let (l0, d) = sbb(a[0], b[0], 0);
    let (l1, d) = sbb(a[1], b[1], d);
    let (l2, d) = sbb(a[2], b[2], d);
    let (l3, d) = sbb(a[3], b[3], d);

    print_hex(&[l0, l1, l2, l3]);

    let (l0, c) = adc(l0, p[0] & d, 0);
    let (l1, c) = adc(l1, p[1] & d, c);
    let (l2, c) = adc(l2, p[2] & d, c);
    let (l3, _) = adc(l3, p[3] & d, c);

    print_hex(&[l0, l1, l2, l3]);

    [l0, l1, l2, l3]
}

pub(crate) fn double(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    add(a, a, p)
}

#[inline]
pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    libc_println!("Mul");
    let (l0, d) = mac(0, a[0], b[0], 0);
    let (l1, d) = mac(0, a[0], b[1], d);
    let (l2, d) = mac(0, a[0], b[2], d);
    let (l3, l4) = mac(0, a[0], b[3], d);

    let (l1, d) = mac(l1, a[1], b[0], 0);
    let (l2, d) = mac(l2, a[1], b[1], d);
    let (l3, d) = mac(l3, a[1], b[2], d);
    let (l4, l5) = mac(l4, a[1], b[3], d);

    let (l2, d) = mac(l2, a[2], b[0], 0);
    let (l3, d) = mac(l3, a[2], b[1], d);
    let (l4, d) = mac(l4, a[2], b[2], d);
    let (l5, l6) = mac(l5, a[2], b[3], d);

    let (l3, d) = mac(l3, a[3], b[0], 0);
    let (l4, d) = mac(l4, a[3], b[1], d);
    let (l5, d) = mac(l5, a[3], b[2], d);
    let (l6, l7) = mac(l6, a[3], b[3], d);

    mont(&[l0, l1, l2, l3, l4, l5, l6, l7], p)
}

pub(crate) fn square(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    libc_println!("Mod = ");
    print_hex(p);
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
    libc_println!("Before mont = {:?}", a);

    let rhs = a[0].wrapping_mul(INV); // s = xk = 6e8e8d63bde23be4

    libc_println!("Rhs = {:x}", rhs);

    let (_, d) = mac(a[0], rhs, p[0], 0); // a + (b * c) + d = 4 + INV * MOD[0] + 0;
    let (l1, d) = mac(a[1], rhs, p[1], d); //
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, e) = adc(a[4], 0, d);

    print_hex(&[l1, l2, l3, l4]);

    let rhs = l1.wrapping_mul(INV);
    libc_println!("Rhs = {:x}", rhs);

    let (_, d) = mac(l1, rhs, p[0], 0);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, e) = adc(a[5], e, d);

    print_hex(&[l1, l2, l3, l4, l5]);

    let rhs = l2.wrapping_mul(INV);
    let (_, d) = mac(l2, rhs, p[0], 0);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, e) = adc(a[6], e, d);

    print_hex(&[l1, l2, l3, l4, l5, l6]);

    let rhs = l3.wrapping_mul(INV);
    let (_, d) = mac(l3, rhs, p[0], 0);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, _) = adc(a[7], e, d);

    print_hex(&[l1, l2, l3, l4, l5, l6, l7]);

    sub(&[l4, l5, l6, l7], p, p)
}

#[cfg(test)]
mod test {
    use libc_print::libc_println;

    use crate::fr::{Fr, MODULUS};

    #[test]
    fn square_test() {
        let x = Fr([2, 0, 0, 0]).square();
        assert!(x < Fr(*MODULUS));
        libc_println!("Real");
        assert_eq!(x, Fr([4, 0, 0, 0]));
    }
}
