#[macro_export]
macro_rules! field_test {
    ($test_name:ident, $field:ident, $iter_times:expr) => {
        paste! {
            #[test]
            fn [< $test_name _comparator_test >]() {
                for _ in 0..$iter_times {
                    let a = $field::random(OsRng);
                    let b = a.square();

                    assert!(a == a);
                    assert!(a >= a);
                    assert!(a <= a);
                    assert!(!(a > a));
                    assert!(!(a < a));
                    assert!(a != b);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _addition_test >]() {
                for _ in 0..$iter_times {
                    let a = $field::random(OsRng);

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
                    let a = $field::random(OsRng);

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
                    let a = $field::random(OsRng);
                    let b = $field::random(OsRng);
                    let c = $field::random(OsRng);

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
                    let a = $field::random(OsRng);
                    let b = $field::random(OsRng);

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
                    let a = $field::random(OsRng);

                    // a * a^-1 = e
                    let inv = a.invert();

                    match inv {
                        Some(x) => {
                            let b = a * x;
                            assert_eq!(b, $field::one())
                        },
                        None => {}
                    }
                }
            }
        }
    };
}

pub use field_test;
