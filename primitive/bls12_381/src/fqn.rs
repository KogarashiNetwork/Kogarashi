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

    fn mul_by_nonres(self) -> Self {
        Self([self.0[0] - self.0[1], self.0[0] + self.0[1]])
    }
}

impl Fq6 {
    fn get_invert(self) -> Option<Self> {
        let mut c0 = self.0[2];
        c0.mul_by_nonresidue();
        c0 *= self.0[1];
        c0 = -c0;
        {
            let c0s = self.0[0];
            c0s.square();
            c0 += c0s;
        }
        let mut c1 = self.0[2];
        c1.square();
        c1.mul_by_nonresidue();
        {
            let mut c01 = self.0[0];
            c01 *= self.0[1];
            c1 -= c01;
        }
        let mut c2 = self.0[1];
        c2.square();
        {
            let mut c02 = self.0[0];
            c02 *= self.0[2];
            c2 -= c02;
        }

        let mut tmp1 = self.0[2];
        tmp1 *= c1;
        let mut tmp2 = self.0[1];
        tmp2 *= c2;
        tmp1 += tmp2;
        tmp1.mul_by_nonresidue();
        tmp2 = self.0[1];
        tmp2 *= c0;
        tmp1 += tmp2;

        match tmp1.invert() {
            Some(t) => Some(Self([t * c0, t * c1, t * c2])),
            None => None,
        }
    }

    fn mul_by_nonres(self) -> Self {
        Self([self.0[2].mul_by_nonresidue(), self.0[0], self.0[1]])
    }
}

impl Fq12 {
    fn get_invert(self) -> Option<Self> {
        let mut c0s = self.0[0];
        c0s.square();
        let c1s = self.0[1];
        c1s.square();
        c1s.mul_by_nonresidue();
        c0s -= c1s;

        c0s.invert()
            .map(|t| Self([t * self.0[0], -(t * self.0[1])]))
    }

    fn mul_by_nonres(self) -> Self {
        unimplemented!()
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
