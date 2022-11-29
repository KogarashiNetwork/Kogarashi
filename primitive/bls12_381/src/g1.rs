use crate::fq::Fq;
use crate::fr::Fr;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Projective {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    pub(crate) z: Fq,
}

const IDENTITY: G1Projective = G1Projective {
    x: Fq::zero(),
    y: Fq::zero(),
    z: Fq::zero(),
};

const GENERATOR: G1Projective = G1Projective {
    x: Fq([
        0x5cb38790fd530c16,
        0x7817fc679976fff5,
        0x154f95c7143ba1c1,
        0xf0ae6acdf3d0e747,
        0xedce6ecc21dbf440,
        0x120177419e0bfb75,
    ]),
    y: Fq([
        0xbaac93d50ce72271,
        0x8c22631a7918fd8e,
        0xdd595f13570725ce,
        0x51ac582950405194,
        0x0e1c8c3fad0059c0,
        0x0bbc3efc5008a26a,
    ]),
    z: Fq::one(),
};

const PARAM_A: Fq = Fq([0, 0, 0, 0, 0, 0]);

const PARAM_B: Fq = Fq([
    0xaa270000000cfff3,
    0x53cc0032fc34000a,
    0x478fe97a6b0a807f,
    0xb1d37ebee6ba24d7,
    0x8ec9733bbf78ab2f,
    0x09d645513d83de7e,
]);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Affine {
    x: Fq,
    y: Fq,
    is_infinity: bool,
}

curve_operation!(
    Fr,
    Fq,
    PARAM_A,
    PARAM_B,
    G1Affine,
    G1Projective,
    GENERATOR,
    IDENTITY
);

#[cfg(test)]
mod tests {
    use super::{
        FftField, Fr, G1Affine, G1Projective, PrimeField, Projective, GENERATOR, IDENTITY,
    };
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_point()(k in arb_fr()) -> G1Projective {
            GENERATOR * k
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn g1_identity_and_generator_test(a in arb_point(), scalar in arb_fr()) {
            // a + (-a) = e
            let e = a - a;

            // a + e = a
            let a_prime = a + IDENTITY;

            // a^0 * g = g
            let g_prime = GENERATOR * scalar.pow(0);

            // a^1 * g = a * g
            let a_power = scalar.pow(1);
            let ag = GENERATOR * scalar;

            assert_eq!(e, IDENTITY);
            assert_eq!(a_prime, a);
            assert_eq!(g_prime, GENERATOR);
            assert_eq!(ag, GENERATOR * a_power);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn g1_add_test(a in arb_point(), b in arb_point(), c in arb_point()) {
            // a + b + c = c + a + b
            let ab = a +b;
            let abc = ab +c;
            let ca = c +a;
            let cab = ca+b;

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
        fn g1_double_test(a in arb_point()) {
            // a + a = a * 8
            let scalared_a = a * Fr([8,0,0,0]);
            let aa = a.double();
            let aaa = aa.double();
            let aaaa = aaa.double();

            assert!(scalared_a.is_on_curve());
            assert!(aaaa.is_on_curve());
            assert_eq!(scalared_a, aaaa);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn g1_scalar_test(g in arb_point()) {
            // 8 * G + 16 * G = 24 * G
            let ag = g * Fr([8, 0, 0, 0]);
            let bg = g * Fr([16, 0, 0, 0]);
            let agbg = ag + bg;

            let abg = g * Fr([24, 0, 0, 0]);

            assert!(agbg.is_on_curve());
            assert!(abg.is_on_curve());
            assert_eq!(agbg, abg);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn g1_conversion_test(a in arb_point()) {
            // projective -> affine -> projective
            let affine = G1Affine::from(a);
            let projective = G1Projective::from(affine);

            assert!(projective.is_on_curve());
            assert_eq!(a, projective);
        }
    }
}
