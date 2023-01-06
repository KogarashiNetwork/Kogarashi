#[macro_export]
macro_rules! curve_test {
    ($test_name:ident, $field:ident, $affine:ident, $projective:ident, $iter_times:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use paste::paste;
            use rand_core::OsRng;

            paste! {
                #[test]
                fn [< $test_name _affine_is_on_curve_test >]() {
                    assert!($affine::ADDITIVE_GENERATOR.is_on_curve());
                    assert!($affine::ADDITIVE_IDENTITY.is_on_curve());
                }
            }

            paste! {
                #[test]
                fn [< $test_name _affine_identity_test >]() {
                    let a = $affine::random(OsRng);
                    // a + (-a) = e
                    let identity = a - a;

                    // a + e = a
                    let a_prime = a + $affine::ADDITIVE_IDENTITY;

                    assert_eq!(identity, $affine::ADDITIVE_IDENTITY);
                    assert_eq!(a_prime, a);
                }
            }

            paste! {
                #[test]
                fn [< $test_name _affine_addition_test >]() {
                    for _ in 0..$iter_times {
                        let a = $affine::random(OsRng);
                        let b = $affine::random(OsRng);
                        let c = $affine::random(OsRng);

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
                fn [< $test_name _affine_doubling_test >]() {
                    for _ in 0..$iter_times {
                        let a = $affine::random(OsRng);

                        // a + a = a * 8
                        let scalared_a = a * $field::from(8 as u64);
                        let a_8 =a.double().double().double();

                        assert!(scalared_a.is_on_curve());
                        assert!(a_8.is_on_curve());
                        assert_eq!(scalared_a, a_8);
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _affine_scalar_test >]() {
                    for _ in 0..$iter_times {
                        let g = $affine::random(OsRng);

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

            paste! {
                #[test]
                fn [< $test_name _affine_conversion_test >]() {
                    for _ in 0..$iter_times {
                        let a = $affine::random(OsRng);

                        // affine -> projective -> affine
                        let projective = a.to_projective();
                        let affine = $affine::from(projective);

                        assert!(affine.is_on_curve());
                        assert_eq!(a, affine);
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _projective_is_on_curve_test >]() {
                    assert!($projective::ADDITIVE_GENERATOR.is_on_curve());
                    assert!($projective::ADDITIVE_IDENTITY.is_on_curve());
                }
            }

            paste! {
                #[test]
                fn [< $test_name _projective_identity_test >]() {
                    let a = $projective::random(OsRng);

                    // a + (-a) = e
                    let identity = a - a;

                    // a + e = a
                    let a_prime = a + $projective::ADDITIVE_IDENTITY;

                    assert_eq!(identity, $projective::ADDITIVE_IDENTITY);
                    assert_eq!(a_prime, a);
                }
            }

            paste! {
                #[test]
                fn [< $test_name _projective_addition_test >]() {
                    for _ in 0..$iter_times {
                        let a = $projective::random(OsRng);
                        let b = $projective::random(OsRng);
                        let c = $projective::random(OsRng);

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
                fn [< $test_name _projective_doubling_test >]() {
                    for _ in 0..$iter_times {
                        let a = $projective::random(OsRng);

                        // a + a = a * 8
                        let scalared_a = a * $field::from(8 as u64);
                        let a_8 =a.double().double().double();

                        assert!(scalared_a.is_on_curve());
                        assert!(a_8.is_on_curve());
                        assert_eq!(scalared_a, a_8);
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _projective_scalar_test >]() {
                    for _ in 0..$iter_times {
                        let g = $projective::random(OsRng);

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

            paste! {
                #[test]
                fn [< $test_name _projective_conversion_test >]() {
                    for _ in 0..$iter_times {
                        let a = $projective::random(OsRng);

                        // projective -> affine -> projective
                        let affine = a.to_affine();
                        let projective = $projective::from(affine);

                        assert!(projective.is_on_curve());
                        assert_eq!(a, projective);
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _mix_addition_test >]() {
                    let a = $affine::random(OsRng);
                    let b = $projective::random(OsRng);
                    let c = $affine::random(OsRng);

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

            paste! {
                #[test]
                fn [< $test_name _mix_doubling_test >]() {
                    for _ in 0..$iter_times {
                        let a = $projective::random(OsRng);
                        let b = a.to_affine();

                        // a + a = a * 8
                        let scalared_a = a * $field::from(8 as u64);
                        let a_8 = a.double().double().double();
                        let b_8 = b.double().double().double();

                        assert!(scalared_a.is_on_curve());
                        assert!(a_8.is_on_curve());
                        assert_eq!(scalared_a, a_8);
                        assert_eq!($affine::from(a_8), b_8);
                    }
                }
            }

            paste! {
                #[test]
                fn [< $test_name _mix_scalar_test >]() {
                    for _ in 0..$iter_times {
                        let g = $projective::random(OsRng);
                        let h = $affine::random(OsRng);

                        // 7 * G + 16 * G = 23 * G
                        let ag = g * $field::from(7 as u64);
                        let bg = g * $field::from(16 as u64);
                        let agbg = ag + bg;

                        let abg = g * $field::from(23 as u64);

                        // 7 * H + 16 * H = 23 * H
                        let ah = h * $field::from(7 as u64);
                        let bh = h * $field::from(16 as u64);
                        let ahbh = ah + bh;

                        let abh = h * $field::from(23 as u64);

                        assert!(agbg.is_on_curve());
                        assert!(abg.is_on_curve());
                        assert_eq!(agbg, abg);
                        assert!(ahbh.is_on_curve());
                        assert!(abh.is_on_curve());
                        assert_eq!(ahbh, abh);
                        assert_eq!($affine::from(agbg), ahbh);
                    }
                }
            }
        }
    };
}

pub use curve_test;
