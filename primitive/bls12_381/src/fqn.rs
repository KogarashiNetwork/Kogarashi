use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use zero_crypto::dress::extension_field::*;
use zero_crypto::dress::pairing::bls12_range_field_pairing;

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 3;
higher_degree_extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
higher_degree_extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENTION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(Fq12, Fq2, G1Affine, PairingCoeff);

impl Fq2 {
    fn get_invert(self) -> Option<Self> {
        match self.is_zero() {
            true => None,
            _ => {
                let t = self.0[0].square() + self.0[1].square();
                let t_inv = t.invert().unwrap();
                Some(Self([t_inv * self.0[0], t_inv * -self.0[1]]))
            }
        }
    }
}

impl Fq6 {
    fn get_invert(self) -> Option<Self> {
        todo!()
    }
}

impl Fq12 {
    fn get_invert(self) -> Option<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Fq2;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::behave::Group;
    use zero_crypto::common::PrimeField;

    prop_compose! {
        fn arb_jubjub_fq2()(bytes in [any::<u8>(); 16]) -> Fq2 {
            Fq2::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn fq2_add_test(a in arb_jubjub_fq2()) {
            // a + a = a * 2
            let b = a + a;
            let c = a.double();
            assert_eq!(b, c);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn fq2_sub_test(a in arb_jubjub_fq2()) {
            // a - a = a * 2 - a * 2
            let b = a - a;
            let c = a.double();
            let d = a.double();
            let e = c - d;

            assert_eq!(b, e);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn fq2_mul_test(a in arb_jubjub_fq2(), b in arb_jubjub_fq2(), c in arb_jubjub_fq2()) {
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn fq2_square_test(a in arb_jubjub_fq2(), b in arb_jubjub_fq2()) {
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn fq2_invert_test(a in arb_jubjub_fq2()) {
            let inv = a.invert();

            match inv {
                Some(x) => {
                    let b = a * x;
                    assert_eq!(b, Fq2::one())
                },
                None => {}
            }
        }
    }
}
