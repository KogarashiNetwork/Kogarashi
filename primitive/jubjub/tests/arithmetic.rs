use zero_jubjub::fr::Fr;

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn add_test(mut a in arb_fr()) {
            let b = a;
            let mut c = a;
            // a + a = a * 2
            a += b;
            c.double_assign();
            assert_eq!(a, c);
        }

    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn sub_test(mut a in arb_fr()) {
            let b = a;
            let mut c = a;
            let mut d = a;

            // a - a = a * 2 - a * 2
            a -= b;
            c.double_assign();

            d.double_assign();
            c -= d;

            assert_eq!(a, c);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn mul_test(mut a in arb_fr(), b in arb_fr(), c in arb_fr()) {
            let mut a2 = a;
            let mut b2 = b;
            let c2 = c;
            let mut a3 = a;

            // a * b + a * c
            a *= b;
            a2 *= c;
            a += a2;

            // a * (b + c)
            b2 += c2;
            a3 *= b2;

            assert_eq!(a, a3);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn square_test(mut a in arb_fr(), mut b in arb_fr()) {
            let mut a2 = a;
            let mut b2 = b;

            // (a * a) * (b * b)
            a *= a;
            b *= b;
            a *= b;

            // a^2 * b^2
            a2.square_assign();
            b2.square_assign();
            a2 *= b2;

            assert_eq!(a, a2);
        }
    }
}
