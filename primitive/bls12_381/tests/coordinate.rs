use proptest::prelude::*;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_bls12_381::{g1::G1Projective, Fr};
use zero_crypto::common::{Group, PrimeField, Projective};

prop_compose! {
    fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
        Fr::random(XorShiftRng::from_seed(bytes))
    }
}

prop_compose! {
    fn arb_point()(k in arb_fr()) -> G1Projective {
        G1Projective::GENERATOR * k
    }
}

#[test]
fn bls12_381_is_on_curve() {
    assert!(G1Projective::GENERATOR.is_on_curve());
    assert!(G1Projective::IDENTITY.is_on_curve());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn bls12_381_identity(a in arb_point()) {
        // a + (-a) = e
        let identity = a - a;

        // a + e = a
        let a_prime = a + G1Projective::IDENTITY;

        assert_eq!(identity, G1Projective::IDENTITY);
        assert_eq!(a_prime, a);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn bls12_381_point_add(a in arb_point(), b in arb_point(), c in arb_point()) {
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn bls12_381_point_double(a in arb_point()) {
        // a + a = a * 8
        let scalared_a = a * Fr::from_u64(8);
        let aa =a.double();
        let a_4 = aa.double();
        let a_8 = a_4.double();

        assert!(scalared_a.is_on_curve());
        assert!(a_8.is_on_curve());
        assert_eq!(scalared_a, a_8);
    }
}
