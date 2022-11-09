mod jubjub;

#[cfg(test)]
mod jubjub_limbs_tests {
    use crate::jubjub::field::*;

    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::arithmetic::limbs::bits_256::*;

    prop_compose! {
        fn arb_jubjub_fr()(bytes in [any::<u8>(); 16]) -> [u64;4] {
            random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn jubjub_field_add_test(a in arb_jubjub_fr()) {
            let b = a;
            let c = a;

            // a + a = a * 2
            let d = add(a, b, MODULUS);
            let e = double(c, MODULUS);
            assert_eq!(d, e);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn jubjub_field_sub_test(a in arb_jubjub_fr()) {
            let b = a;
            let c = a;
            let d = a;

            // a - a = a * 2 - a * 2
            let e = sub(a, b, MODULUS);

            let cc = double(c, MODULUS);
            let dd = double(d, MODULUS);
            let f = sub(cc, dd, MODULUS);

            assert_eq!(e, f);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn jubjub_field_mul_test(a in arb_jubjub_fr(), b in arb_jubjub_fr(), c in arb_jubjub_fr()) {
            // a * b + a * c
            let ab = mul(a, b, MODULUS, INV);
            let ac = mul(a, c, MODULUS, INV);
            let d = add(ab, ac, MODULUS);

            // a * (b + c)
            let bc = add(b, c, MODULUS);
            let e = mul(a, bc, MODULUS, INV);

            assert_eq!(d, e);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn jubjub_field_square_test(a in arb_jubjub_fr(), b in arb_jubjub_fr()) {
            // (a * a) * (b * b)
            let aa = mul(a, a, MODULUS, INV);
            let bb = mul(b, b, MODULUS, INV);
            let c = mul(aa, bb, MODULUS, INV);

            // a^2 * b^2
            let aa = square(a, MODULUS, INV);
            let bb = square(b, MODULUS, INV);
            let d = mul(aa, bb, MODULUS, INV);

            assert_eq!(c, d);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn jubjub_field_invert_test(a in arb_jubjub_fr()) {
            let one = from_raw([1,0,0,0]);
            let inv = invert(a, MODULUS, INV);

            match inv {
                Some(x) => {
                    let b = mul(a, x, MODULUS, INV);
                    assert_eq!(b, one)
                }
                None => {}
            }
        }
    }
}
