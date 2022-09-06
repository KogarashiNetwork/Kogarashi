use crate::fr::Fr;

use super::utils::{adc, mac, sbb};
const INV: u64 = 0x1ba3_a358_ef78_8ef9;

#[inline]
pub(crate) fn invert(x: &Fr) -> Option<[u64; 4]> {
    #[inline(always)]
    fn square_assign_multi(n: &mut Fr, num_times: usize) {
        for _ in 0..num_times {
            *n = n.square();
        }
    }
    let mut t1 = x.square();
    let mut t0 = t1.square();
    let mut t3 = t0 * t1;
    let t6 = t3 * *x;
    let t7 = t6 * t1;
    let t12 = t7 * t3;
    let t13 = t12 * t0;
    let t16 = t12 * t3;
    let t2 = t13 * t3;
    let t15 = t16 * t3;
    let t19 = t2 * t0;
    let t9 = t15 * t3;
    let t18 = t9 * t3;
    let t14 = t18 * t1;
    let t4 = t18 * t0;
    let t8 = t18 * t3;
    let t17 = t14 * t3;
    let t11 = t8 * t3;
    t1 = t17 * t3;
    let t5 = t11 * t3;
    t3 = t5 * t0;
    t0 = t5.square();
    square_assign_multi(&mut t0, 5);
    t0 *= t3;
    square_assign_multi(&mut t0, 6);
    t0 *= t8;
    square_assign_multi(&mut t0, 7);
    t0 *= t19;
    square_assign_multi(&mut t0, 6);
    t0 *= t13;
    square_assign_multi(&mut t0, 8);
    t0 *= t14;
    square_assign_multi(&mut t0, 6);
    t0 *= t18;
    square_assign_multi(&mut t0, 7);
    t0 *= t17;
    square_assign_multi(&mut t0, 5);
    t0 *= t16;
    square_assign_multi(&mut t0, 3);
    t0 *= *x;
    square_assign_multi(&mut t0, 11);
    t0 *= t11;
    square_assign_multi(&mut t0, 8);
    t0 *= t5;
    square_assign_multi(&mut t0, 5);
    t0 *= t15;
    square_assign_multi(&mut t0, 8);
    t0 *= *x;
    square_assign_multi(&mut t0, 12);
    t0 *= t13;
    square_assign_multi(&mut t0, 7);
    t0 *= t9;
    square_assign_multi(&mut t0, 5);
    t0 *= t15;
    square_assign_multi(&mut t0, 14);
    t0 *= t14;
    square_assign_multi(&mut t0, 5);
    t0 *= t13;
    square_assign_multi(&mut t0, 2);
    t0 *= *x;
    square_assign_multi(&mut t0, 6);
    t0 *= *x;
    square_assign_multi(&mut t0, 9);
    t0 *= t7;
    square_assign_multi(&mut t0, 6);
    t0 *= t12;
    square_assign_multi(&mut t0, 8);
    t0 *= t11;
    square_assign_multi(&mut t0, 3);
    t0 *= *x;
    square_assign_multi(&mut t0, 12);
    t0 *= t9;
    square_assign_multi(&mut t0, 11);
    t0 *= t8;
    square_assign_multi(&mut t0, 8);
    t0 *= t7;
    square_assign_multi(&mut t0, 4);
    t0 *= t6;
    square_assign_multi(&mut t0, 10);
    t0 *= t5;
    square_assign_multi(&mut t0, 7);
    t0 *= t3;
    square_assign_multi(&mut t0, 6);
    t0 *= t4;
    square_assign_multi(&mut t0, 7);
    t0 *= t3;
    square_assign_multi(&mut t0, 5);
    t0 *= t2;
    square_assign_multi(&mut t0, 6);
    t0 *= t2;
    square_assign_multi(&mut t0, 7);
    t0 *= t1;

    match x.is_zero() {
        true => None,
        false => Some(t0.0),
    }
}

#[inline]
pub(crate) const fn add(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);

    sub(&[l0, l1, l2, l3], p, p)
}

#[inline]
pub(crate) const fn sub(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
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
pub(crate) const fn double(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    add(a, a, p)
}

#[inline]
pub(crate) const fn mul(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
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

#[inline]
pub(crate) const fn square(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    mul(a, a, p)
}

#[inline]
pub(crate) fn neg(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    if a == &[0; 4] {
        *a
    } else {
        sub(p, a, p)
    }
}

#[inline]
pub(crate) const fn mont(a: &[u64; 8], p: &[u64; 4]) -> [u64; 4] {
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
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use crate::fr::{Fr, MODULUS};

    use super::{invert, mul};

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn test_invert(x in arb_fr()) {
            match invert(&x) {
                Some(y) => {
                    let z = mul(&x.0, &y, MODULUS);
                    assert_eq!(Fr(z), Fr::one());
                },
                None => assert_eq!(x, Fr::zero())
            }

        }
    }
}
