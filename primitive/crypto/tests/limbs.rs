mod bls12_381;
mod jubjub;

use proptest::prelude::*;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

#[cfg(test)]
mod jubjub_limbs_tests {
    use super::*;
    use crate::jubjub::field::*;
    use zero_crypto::arithmetic::bits_256::*;

    prop_compose! {
        fn arb_jubjub_fr()(bytes in [any::<u8>(); 16]) -> [u64; 4] {
            random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
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
        #![proptest_config(ProptestConfig::with_cases(100000))]
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
        #![proptest_config(ProptestConfig::with_cases(10000))]
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
        #![proptest_config(ProptestConfig::with_cases(10000))]
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
            let one = from_raw([1, 0, 0, 0]);
            let inv = invert(a, sub(zero(), [2, 0, 0, 0], MODULUS), one, MODULUS, INV);

            match inv {
                Some(x) => {
                    let b = mul(a, x, MODULUS, INV);
                    assert_eq!(b, one)
                }
                None => {}
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn jubjub_field_power_test(a in arb_jubjub_fr()) {
            let one = from_raw([1, 0, 0, 0]);
            let identity = pow(a, sub(zero(), [1, 0, 0, 0], MODULUS), one, MODULUS, INV);
            assert_eq!(one, identity)
        }
    }
}

#[cfg(test)]
mod bls12_381_limbs_tests {
    use super::*;
    use crate::bls12_381::field::*;
    use zero_crypto::arithmetic::bits_384::*;

    prop_compose! {
        fn arb_bls12_381_fp()(bytes in [any::<u8>(); 16]) -> [u64; 6] {
            random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn bls12_381_field_add_test(a in arb_bls12_381_fp()) {
            let b = a;
            let c = a;

            // a + a = a * 2
            let d = add(a, b, MODULUS);
            let e = double(c, MODULUS);
            assert_eq!(d, e);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn bls12_381_field_sub_test(a in arb_bls12_381_fp()) {
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
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn bls12_381_field_mul_test(a in arb_bls12_381_fp(), b in arb_bls12_381_fp(), c in arb_bls12_381_fp()) {
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
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn bls12_381_field_square_test(a in arb_bls12_381_fp(), b in arb_bls12_381_fp()) {
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
        fn bls12_381_field_invert_test(a in arb_bls12_381_fp()) {
            let one = from_raw([1,0,0,0,0,0]);
            let little_fermat = sub(MODULUS, [2,0,0,0,0,0], MODULUS);
            let inv = invert(a, little_fermat, one, MODULUS, INV);

            match inv {
                Some(x) => {
                    let b = mul(a, x, MODULUS, INV);
                    assert_eq!(b, one)
                }
                None => {}
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn jubjub_field_power_test(a in arb_bls12_381_fp()) {
            let one = from_raw([1, 0, 0, 0, 0, 0]);
            let identity = pow(a, sub(zero(), [1, 0, 0, 0, 0, 0], MODULUS), one, MODULUS, INV);
            assert_eq!(one, identity)
        }
    }
}
