mod bls12_381;
mod jubjub;

use proptest::prelude::*;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_crypto::arithmetic::utils::ProjectiveCoordinate;

#[cfg(test)]
mod jubjub_curve_tests {
    use super::*;
    use crate::jubjub::curve::*;
    use crate::jubjub::field::*;
    use zero_crypto::arithmetic::bits_256::*;

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

            // 2 * (a + b) = 2 * a + 2 * b
            let double_ab = double_point(ab, MODULUS, INV);
            let aa = double_point(a, MODULUS, INV);
            let bb = double_point(b, MODULUS, INV);
            let aabb = add_point(aa, bb, MODULUS, INV);

            assert!(is_on_curve(abc));
            assert!(is_on_curve(cab));
            assert!(is_on_curve(double_ab));
            assert!(is_on_curve(aabb));
            assert_eq!(abc, cab);
            assert_eq!(double_ab, aabb);
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
    use super::*;
    use crate::bls12_381::curve::*;
    use crate::bls12_381::field::*;
    use zero_crypto::arithmetic::bits_384::*;

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

            // 2 * (a + b) = 2 * a + 2 * b
            let double_ab = double_point(ab, MODULUS, INV);
            let aa = double_point(a, MODULUS, INV);
            let bb = double_point(b, MODULUS, INV);
            let aabb = add_point(aa, bb, MODULUS, INV);

            assert!(is_on_curve(abc));
            assert!(is_on_curve(cab));
            assert!(is_on_curve(double_ab));
            assert!(is_on_curve(aabb));
            assert_eq!(abc, cab);
            assert_eq!(double_ab, aabb);
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
