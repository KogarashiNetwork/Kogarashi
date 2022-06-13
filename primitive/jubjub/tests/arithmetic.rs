use zero_jubjub::Fr;

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
            let b = a.clone();
            let mut c = a.clone();
            // a + a = a * 2
            a.add_assign(b);
            c.double_assign();
            assert_eq!(a, c);
        }

    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn sub_test(mut a in arb_fr()) {
            let b = a.clone();
            let mut c = a.clone();
            let mut d = a.clone();

            // a - a = a * 2 - a * 2
            a.sub_assign(b);
            c.double_assign();

            d.double_assign();
            c.sub_assign(d);

            assert_eq!(a, c);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn mul_test(mut a in arb_fr(), b in arb_fr(), c in arb_fr()) {
            let mut a2 = a.clone();
            let mut b2 = b.clone();
            let c2 = c.clone();
            let mut a3 = a.clone();

            // a * b + a * c
            a.mul_assign(b);
            a2.mul_assign(c);
            a.add_assign(a2);

            // a * (b + c)
            b2.add_assign(c2);
            a3.mul_assign(b2);

            assert_eq!(a, a3);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]
        #[test]
        fn square_test(mut a in arb_fr(), mut b in arb_fr()) {
            let mut a2 = a.clone();
            let mut b2 = b.clone();

            // (a * a) * (b * b)
            a.mul_assign(a.clone());
            b.mul_assign(b.clone());
            a.mul_assign(b);

            // a^2 * b^2
            a2.square_assign();
            b2.square_assign();
            a2.mul_assign(b2);

            assert_eq!(a, a2);
        }
    }
}
