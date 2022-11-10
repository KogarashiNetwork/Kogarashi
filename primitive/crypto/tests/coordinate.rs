mod bls12_381;
mod jubjub;

#[cfg(test)]
mod jubjub_curve_tests {
    use crate::jubjub::curve::*;
    use crate::jubjub::field::*;

    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::arithmetic::{
        coordinate::projective::*, coordinate::utils::*, limbs::bits_256::*,
    };

    prop_compose! {
        fn arb_jubjub_point()(bytes in [any::<u8>(); 16]) -> ProjectiveCoordinate<[u64; 4]> {
            random_point(XorShiftRng::from_seed(bytes))
        }
    }

    #[test]
    fn jubjub_curve_is_on_curve_test() {
        assert!(is_on_curve(GENERATOR));
        assert!(is_on_curve(IDENTITY));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn jubjub_curve_identity(a in arb_jubjub_point()) {
            // a + (-a) = e
            let (x, y, z) = a;
            let b = (x, neg(y, MODULUS), z);
            let identity = add_point(a, b, MODULUS, INV);

            // a + e = a
            let a_prime = add_point(a, IDENTITY, MODULUS, INV);

            assert_eq!(identity, IDENTITY);
            assert_eq!(a_prime, a);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn jubjub_curve_add_test(a in arb_jubjub_point(), b in arb_jubjub_point(), c in arb_jubjub_point()) {
            // a + b + c = c + a + b
            let ab = add_point(a, b, MODULUS, INV);
            let abc = add_point(ab, c, MODULUS, INV);
            let ca = add_point(c, a, MODULUS, INV);
            let cab = add_point(ca,b, MODULUS, INV);

            assert!(is_on_curve(abc));
            assert!(is_on_curve(cab));
            assert_eq!(abc, cab);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn jubjub_curve_double_test(a in arb_jubjub_point()) {
            // a + a = a * 8
            let scalared_a = scalar_point(a, [8,0,0,0], IDENTITY, MODULUS, INV);
            let aa = double_point(a, MODULUS, INV);
            let aaa = double_point(aa, MODULUS, INV);
            let aaaa = double_point(aaa, MODULUS, INV);

            assert!(is_on_curve(scalared_a));
            assert!(is_on_curve(aaaa));
            assert_eq!(scalared_a, aaaa);
        }
    }
}

#[cfg(test)]
mod bls12_381_curve_tests {
    use crate::bls12_381::curve::*;
    use crate::bls12_381::field::*;

    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::arithmetic::{
        coordinate::bits_384::projective::*, coordinate::utils::*, limbs::bits_384::*,
    };

    prop_compose! {
        fn arb_bls12_381_point()(bytes in [any::<u8>(); 16]) -> ProjectiveCoordinate<[u64; 6]> {
            random_point(XorShiftRng::from_seed(bytes))
        }
    }

    #[test]
    fn bls12_381_curve_is_on_curve_test() {
        assert!(is_on_curve(GENERATOR));
        assert!(is_on_curve(IDENTITY));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn bls12_381_curve_identity(a in arb_bls12_381_point()) {
            // a + (-a) = e
            let (x, y, z) = a;
            let b = (x, neg(y, MODULUS), z);
            let identity = add_point(a, b, MODULUS, INV);

            // a + e = a
            let a_prime = add_point(a, IDENTITY, MODULUS, INV);

            assert_eq!(identity, IDENTITY);
            assert_eq!(a_prime, a);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn bls12_381_curve_add_test(a in arb_bls12_381_point(), b in arb_bls12_381_point(), c in arb_bls12_381_point()) {
            // a + b + c = c + a + b
            let ab = add_point(a, b, MODULUS, INV);
            let abc = add_point(ab, c, MODULUS, INV);
            let ca = add_point(c, a, MODULUS, INV);
            let cab = add_point(ca,b, MODULUS, INV);

            assert!(is_on_curve(abc));
            assert!(is_on_curve(cab));
            assert_eq!(abc, cab);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn jubjub_curve_double_test(a in arb_bls12_381_point()) {
            // a + a = a * 8
            let scalared_a = scalar_point(a, [8,0,0,0,0,0], IDENTITY, MODULUS, INV);
            let aa = double_point(a, MODULUS, INV);
            let aaa = double_point(aa, MODULUS, INV);
            let aaaa = double_point(aaa, MODULUS, INV);

            assert!(is_on_curve(scalared_a));
            assert!(is_on_curve(aaaa));
            assert_eq!(scalared_a, aaaa);
        }
    }
}
