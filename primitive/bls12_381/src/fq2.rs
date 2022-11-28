use crate::fq::Fq;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::dress::extention_field::*;

// sextic twist of Fp12
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fq2(pub(crate) [Fq; 2]);

const ZERO: Fq2 = Fq2([Fq([0, 0, 0, 0, 0, 0]), Fq([0, 0, 0, 0, 0, 0])]);

extention_field_operation!(Fq2, Fq, ZERO);

#[cfg(test)]
mod tests {
    use super::Fq2;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::common::PrimeField;

    prop_compose! {
        fn arb_jubjub_fq()(bytes in [any::<u8>(); 16]) -> Fq2 {
            Fq2::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn fq_add_test(a in arb_jubjub_fq()) {
            // a + a = a * 2
            let b = a + a;
            let c = a.double();
            assert_eq!(b, c);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn fq_sub_test(a in arb_jubjub_fq()) {
            // a - a = a * 2 - a * 2
            let b = a - a;
            let c = a.double();
            let d = a.double();
            let e = c - d;

            assert_eq!(b, e);
        }
    }
}
