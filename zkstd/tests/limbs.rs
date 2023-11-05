mod construction;

macro_rules! limbs_test {
    ($test_name:ident, $test_bits:ident, $test_mod:ident, $limbs_type:ident, $one:expr, $two:expr) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;
            use construction::$test_mod::*;
            use paste::paste;
            use rand_core::OsRng;
            use zkstd::arithmetic::$test_bits::*;

            fn arb_field() -> $limbs_type {
                random(OsRng)
            }

            paste! {
                #[test]
                fn [< $test_name _add_test >]() {
                    let a = arb_field();
                    let b = a;
                    let c = a;

                    // a + a = a * 2
                    let d = add(a, b, MODULUS);
                    let e = double(c, MODULUS);
                    assert_eq!(d, e);
                }
            }

            paste! {
                #[test]
                fn [< $test_name _sub_test >]() {
                    let a = arb_field();
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

            paste! {
            #[test]
                fn [< $test_name _mul_test >]() {
                    let a = arb_field();
                    let b = arb_field();
                    let c = arb_field();

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

            paste! {
                #[test]
                fn [< $test_name _square_test >]() {
                    let a = arb_field();
                    let b = arb_field();

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

            paste! {
                #[test]
                fn [< $test_name _invert_test >]() {
                    let a = arb_field();
                    let one = from_raw($one);
                    let inv = invert(a, sub(zero(), $two, MODULUS), one, MODULUS, INV);

                    if let Some(x) = inv {
                        let b = mul(a, x, MODULUS, INV);
                        assert_eq!(b, one)
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _power_test >]() {
                    let a = arb_field();
                    let one = from_raw($one);
                    let identity = pow(a, sub(zero(), $one, MODULUS), one, MODULUS, INV);
                    let zero_power = pow(a, zero(), one, MODULUS, INV);

                    assert_eq!(one, identity);
                    assert_eq!(one, zero_power);
                }
            }
        }
    };
}

use construction::{Bits256Limbs, Bits384Limbs};

limbs_test!(
    jubjub_limbs_tests,
    bits_256,
    jubjub_field,
    Bits256Limbs,
    [1, 0, 0, 0],
    [2, 0, 0, 0]
);
limbs_test!(
    bls12_381_limbs_tests,
    bits_384,
    bls12_381_field,
    Bits384Limbs,
    [1, 0, 0, 0, 0, 0],
    [2, 0, 0, 0, 0, 0]
);
