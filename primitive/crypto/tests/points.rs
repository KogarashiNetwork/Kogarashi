mod construction;

#[cfg(test)]
mod twisted_edwards_points_tests {
    use super::*;
    use construction::jubjub_curve::{BlsScalar, JubjubAffine};
    use rand_core::OsRng;
    use zero_crypto::{
        arithmetic::edwards::{add_point, double_point},
        common::Curve,
    };

    #[test]
    fn is_on_curve() {
        let g = JubjubAffine::ADDITIVE_GENERATOR;
        let e = JubjubAffine::ADDITIVE_IDENTITY;
        let a = JubjubAffine::random(OsRng);
        let b = a + g;
        let c = b + e;

        assert!(g.is_on_curve());
        assert!(e.is_on_curve());
        assert!(a.is_on_curve());
        assert!(b.is_on_curve());
        assert!(c.is_on_curve());
    }

    #[test]
    fn addition_test() {
        let a = JubjubAffine::random(OsRng);
        let b = JubjubAffine::random(OsRng);

        // 2 * (a + b) = 2 * a + 2 * b
        let c = double_point(add_point(a, b));
        let d = add_point(double_point(a), double_point(b));

        assert_eq!(c, d);
    }

    #[test]
    fn scalar_test() {
        let r = BlsScalar::to_mont_form([9, 0, 0, 0]);
        let a = JubjubAffine::random(OsRng);

        // (2 * 2 * 2 * b) + b = 9 * b
        let b = add_point(a, double_point(double_point(double_point(a))));
        let c = a * r;

        assert_eq!(b, c);
    }
}
