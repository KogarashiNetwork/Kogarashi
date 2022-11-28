use crate::fp::Fp;
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubProjective {
    pub(crate) x: Fp,
    pub(crate) y: Fp,
    pub(crate) z: Fp,
}

const IDENTITY: JubjubProjective = JubjubProjective {
    x: Fp::zero(),
    y: Fp::zero(),
    z: Fp::zero(),
};

const GENERATOR: JubjubProjective = JubjubProjective {
    x: Fp::to_mont_form([
        0x7c24d812779a3316,
        0x72e38f4ebd4070f3,
        0x03b3fe93f505a6f2,
        0xc4c71e5a4102960,
    ]),
    y: Fp::to_mont_form([
        0xd2047ef3463de4af,
        0x01ca03640d236cbf,
        0xd3033593ae386e92,
        0xaa87a50921b80ec,
    ]),
    z: Fp::one(),
};

const PARAM_A: Fp = Fp::zero();

const PARAM_B: Fp = Fp::to_mont_form([4, 0, 0, 0]);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubAffine {
    x: Fp,
    y: Fp,
    is_infinity: bool,
}

curve_operation!(
    Fp,
    Fp,
    PARAM_A,
    PARAM_B,
    JubjubAffine,
    JubjubProjective,
    GENERATOR,
    IDENTITY
);

#[cfg(test)]
mod tests {
    use super::{Fp, JubjubProjective, PrimeField, GENERATOR};
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    prop_compose! {
        fn arb_fp()(bytes in [any::<u8>(); 16]) -> Fp {
            Fp::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_cdn()(k in arb_fp()) -> JubjubProjective {
            GENERATOR * k
        }
    }

    #[test]
    fn test_coordinate_cmp() {
        let a = JubjubProjective {
            x: Fp::one(),
            y: Fp::one(),
            z: Fp::one(),
        };
        let b = JubjubProjective {
            x: Fp::one(),
            y: Fp::zero(),
            z: Fp::one(),
        };
        assert_ne!(a, b)
    }
}
