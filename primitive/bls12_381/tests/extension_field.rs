use paste::paste;
use rand_core::OsRng;
use zero_bls12_381::{Fq12, Fq2, Fq6};
use zero_crypto::behave::{Group, PrimeField};

fn arb_ext_fq<F: PrimeField>() -> F {
    F::random(OsRng)
}

macro_rules! bls12_extension_field_test {
    ($test_name:ident, $ext_field:ident, $iter_times:expr) => {
        paste! {
            #[test]
            fn [< $test_name _addition_test >]() {
                for _ in 0..$iter_times {
                    let a = arb_ext_fq::<$ext_field>();

                    // a + a = a * 2
                    let b = a + a;
                    let c = a.double();

                    assert_eq!(b, c);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _subtraction_test >]() {
                for _ in 0..$iter_times {
                    let a = arb_ext_fq::<$ext_field>();

                    // a - a = a * 2 - a * 2
                    let b = a - a;
                    let c = a.double();
                    let d = a.double();
                    let e = c - d;

                    assert_eq!(b, e);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _multiplication_test >]() {
                for _ in 0..$iter_times {
                    let a = arb_ext_fq::<$ext_field>();
                    let b = arb_ext_fq::<$ext_field>();
                    let c = arb_ext_fq::<$ext_field>();

                    // a * b + a * c
                    let ab = a * b;
                    let ac = a * c;
                    let d = ab + ac;

                    // a * (b + c)
                    let bc = b + c;
                    let e = a * bc;

                    assert_eq!(d, e);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _squaring_test >]() {
                for _ in 0..$iter_times {
                    let a = arb_ext_fq::<$ext_field>();
                    let b = arb_ext_fq::<$ext_field>();

                    // (a * a) * (b * b)
                    let aa = a * a;
                    let bb = b * b;
                    let c = aa * bb;

                    // a^2 * b^2
                    let aa = a.square();
                    let bb = b.square();
                    let d = aa * bb;

                    assert_eq!(c, d);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _inversion_test >]() {
                for _ in 0..$iter_times {
                    let a = arb_ext_fq::<$ext_field>();

                    // a * a^-1 = e
                    let inv = a.invert();

                    match inv {
                        Some(x) => {
                            let b = a * x;
                            assert_eq!(b, $ext_field::one())
                        },
                        None => {}
                    }
                }
            }
        }
    };
}

bls12_extension_field_test!(fq2_field, Fq2, 1000);
bls12_extension_field_test!(fq6_field, Fq6, 500);
bls12_extension_field_test!(fq12_field, Fq12, 500);
