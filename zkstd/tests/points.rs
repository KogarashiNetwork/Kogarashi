mod jubjub;

#[cfg(test)]
mod twisted_edwards_points_tests {
    use super::*;
    use jubjub::jubjub_curve::{BlsScalar, JubjubAffine, JubjubExtended};
    use rand_core::OsRng;
    use zkstd::{
        arithmetic::edwards::*,
        common::{CurveGroup, Group, TwistedEdwardsAffine},
    };

    #[test]
    fn is_on_curve_affine() {
        let mut rng = OsRng;
        let g = JubjubAffine::ADDITIVE_GENERATOR;
        let e = JubjubAffine::ADDITIVE_IDENTITY;
        let a = JubjubAffine::random(&mut rng);
        let b = a + g;
        let c = b + e;

        assert!(g.is_on_curve());
        assert!(e.is_on_curve());
        assert!(a.is_on_curve());
        assert!(b.is_on_curve());
        assert!(c.is_on_curve());
    }

    #[test]
    fn is_on_curve_extended() {
        let mut rng = OsRng;
        let g = JubjubExtended::ADDITIVE_GENERATOR;
        let e = JubjubExtended::ADDITIVE_IDENTITY;
        let a = JubjubExtended::random(&mut rng);
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
        let mut rng = OsRng;
        let a = JubjubAffine::random(&mut rng);
        let b = JubjubAffine::random(&mut rng);

        // 2 * (a + b) = 2 * a + 2 * b
        let c = double_projective_point(add_affine_point(a, b));
        let d = add_projective_point(double_affine_point(a), double_affine_point(b));

        assert_eq!(c, d);
    }

    #[test]
    fn scalar_test() {
        let mut rng = OsRng;
        let r = BlsScalar::to_mont_form([9, 0, 0, 0]);
        let a = JubjubAffine::random(&mut rng).to_extended();

        // (2 * 2 * 2 * b) + b = 9 * b
        let b = add_projective_point(
            a,
            double_projective_point(double_projective_point(double_projective_point(a))),
        );
        let c = a * r;

        assert_eq!(b, c);
    }
}
