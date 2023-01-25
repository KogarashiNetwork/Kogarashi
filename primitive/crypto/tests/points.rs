mod construction;

#[cfg(test)]
mod jubjub_points_tests {
    use super::*;
    use construction::jubjub_curve::{JubjubAffine, JubjubExtend};
    use zero_crypto::common::Curve;

    #[test]
    fn is_on_curve() {
        let g = JubjubAffine::ADDITIVE_GENERATOR;
        let e = JubjubAffine::ADDITIVE_IDENTITY;

        assert!(g.is_on_curve());
        assert!(e.is_on_curve());
    }
}
