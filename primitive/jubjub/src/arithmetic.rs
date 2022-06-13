mod assembly;
mod normal;
mod utils;

#[cfg(all(feature = "normal"))]
pub(crate) use normal::{add, double, mul, neg, square, sub};

pub(crate) use assembly::{add, double, mul, neg, square, sub};

#[cfg(test)]
mod test {
    use super::assembly::{
        add as asm_add, double as asm_double, mul as asm_mul, neg as asm_neg, square as asm_square,
        sub as asm_sub,
    };
    use super::normal::{add, double, mul, neg, square, sub};
    use crate::entity::Fr;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn test_normal_and_asm(a in arb_fr(), b in arb_fr()) {
            assert_eq!(add(&a.0, &b.0), asm_add(&a.0, &b.0));
            assert_eq!(sub(&a.0, &b.0), asm_sub(&a.0, &b.0));
            assert_eq!(double(&a.0), asm_double(&a.0));
            assert_eq!(neg(&a.0), asm_neg(&a.0));
            // assert_eq!(square(&a.0), asm_square(&a.0));
            // assert_eq!(mul(&a.0, &b.0), asm_mul(&a.0, &b.0));
        }
    }
}
