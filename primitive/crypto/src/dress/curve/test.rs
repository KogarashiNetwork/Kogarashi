#[macro_export]
macro_rules! curve_test {
    ($test_name:ident, $field:ident, $affine:ident, $projective:ident, $iter_times:expr) => {
        use super::*;
        use paste::paste;
        use rand_core::OsRng;

        curve_operation_test!($test_name, affine, $affine, $field, $iter_times);
        curve_operation_test!($test_name, projective, $projective, $field, $iter_times);

        paste! {
            #[test]
            fn [< $test_name _coordinate_transformation_test >]() {
                for _ in 0..$iter_times {
                    let a = $affine::from($affine::random(OsRng));
                    let b = $projective::from(a);

                    // projective -> affine -> projective
                    let projective = $projective::from(a);
                    let affine = $affine::from(b);

                    assert!(affine.is_on_curve());
                    assert!(projective.is_on_curve());
                    assert_eq!(a, affine);
                    assert_eq!(b, projective);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _mix_addition_test >]() {
                let a = $affine::random(OsRng);
                let b = $affine::random(OsRng);
                let c = $affine::random(OsRng);
                let d = $projective::from(a);
                let e = $projective::from(b);
                let f = $projective::from(c);

                // a + b + c = c + a + b
                // d + e + f = f + d + e
                let abc = a + b + c;
                let cab = c + a + b;
                let def = d + e + f;
                let fde = f + d + e;

                // 2 * (a + b) = 2 * a + 2 * b
                // 2 * (d + e) = 2 * d + 2 * e
                let double_ab = (a + b).double();
                let aabb = a.double() + b.double();
                let double_de = (d + e).double();
                let ddee = d.double() + e.double();

                assert_eq!(abc, cab);
                assert_eq!(def, fde);
                assert_eq!(double_ab, aabb);
                assert_eq!(double_de, ddee);

                // projective and affine test
                assert_eq!($projective::from(abc), fde);
                assert_eq!($projective::from(cab), def);
                assert_eq!($projective::from(double_ab), ddee);
                assert_eq!($projective::from(aabb), double_de);
            }
        }

        paste! {
            #[test]
            fn [< $test_name _mix_doubling_test >]() {
                for _ in 0..$iter_times {
                    let a = $affine::random(OsRng);
                    let b = $projective::from(a);
                    let s = $field::from(8 as u64);

                    // a + a = a * 8
                    // b + b = b * 8
                    let scalared_a = a * s;
                    let a_8 = a.double().double().double();

                    let scalared_b = b * s;
                    let b_8 = b.double().double().double();

                    assert_eq!(scalared_a, a_8);
                    assert_eq!(scalared_b, b_8);

                    // projective and affine test
                    assert_eq!($projective::from(scalared_a), b_8);
                    assert_eq!($projective::from(a_8), scalared_b);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _mix_scalar_test >]() {
                for _ in 0..$iter_times {
                    let g = $affine::random(OsRng);
                    let h = $projective::from(g);

                    // 2 * (7 * G + 16 * G) = 46 * G
                    let ag = g * $field::from(7 as u64);
                    let bg = g * $field::from(16 as u64);
                    let agbg = (ag + bg).double();

                    let abg = g * $field::from(46 as u64);

                    // 2 * (7 * H + 16 * H) = 46 * H
                    let ah = h * $field::from(7 as u64);
                    let bh = h * $field::from(16 as u64);
                    let ahbh = (ah + bh).double();

                    let abh = h * $field::from(46 as u64);

                    assert_eq!(agbg, abg);
                    assert_eq!(ahbh, abh);

                    // projective and affine test
                    assert_eq!($projective::from(agbg), abh);
                    assert_eq!($projective::from(abg), ahbh);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! curve_operation_test {
    ($test_name:ident, $curve_name:ident, $curve:ident, $field:ident, $iter_times:expr) => {
        paste! {
            #[test]
            fn [< $test_name _ $curve_name _is_on_curve_test >]() {
                assert!($curve::ADDITIVE_GENERATOR.is_on_curve());
                assert!($curve::ADDITIVE_IDENTITY.is_on_curve());
            }
        }

        paste! {
            #[test]
            fn [< $test_name _ $curve_name _identity_test >]() {
                let a = $curve::random(OsRng);
                // a + e = a
                let a_prime = a + $curve::ADDITIVE_IDENTITY;

                assert_eq!(a_prime, a);
            }
        }

        paste! {
            #[test]
            fn [< $test_name _ $curve_name _addition_test >]() {
                for _ in 0..$iter_times {
                    let a = $curve::random(OsRng);
                    let b = $curve::random(OsRng);
                    let c = $curve::random(OsRng);

                    // a + b + c = c + a + b
                    let ab = a + b;
                    let abc = ab + c;
                    let ca = c + a;
                    let cab = ca + b;

                    // 2 * (a + b) = 2 * a + 2 * b
                    let double_ab = ab.double();
                    let aa = a.double();
                    let bb = b.double();
                    let aabb = aa + bb;

                    assert!(abc.is_on_curve());
                    assert!(cab.is_on_curve());
                    assert!(double_ab.is_on_curve());
                    assert!(aabb.is_on_curve());
                    assert_eq!(abc, cab);
                    assert_eq!(double_ab, aabb);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _ $curve_name _doubling_test >]() {
                for _ in 0..$iter_times {
                    let a = $curve::random(OsRng);

                    // a + a = a * 8
                    let scalared_a = a * $field::from(8 as u64);
                    let a_8 = a.double().double().double();

                    assert!(scalared_a.is_on_curve());
                    assert!(a_8.is_on_curve());
                    assert_eq!(scalared_a, a_8);
                }
            }
        }

        paste! {
            #[test]
            fn [< $test_name _ $curve_name _scalar_test >]() {
                for _ in 0..$iter_times {
                    let g = $curve::random(OsRng);

                    // 7 * G + 16 * G = 23 * G
                    let ag = g * $field::from(7 as u64);
                    let bg = g * $field::from(16 as u64);
                    let agbg = ag + bg;

                    let abg = g * $field::from(23 as u64);

                    assert!(agbg.is_on_curve());
                    assert!(abg.is_on_curve());
                    assert_eq!(agbg, abg);
                }
            }
        }
    };
}

pub use {curve_operation_test, curve_test};
