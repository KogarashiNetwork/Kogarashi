mod assembly;
mod normal;
mod utils;

#[cfg(all(feature = "normal"))]
pub(crate) use normal::{add, double, mul, neg, square, sub};

pub(crate) use assembly::{add, double, mul, neg, square, sub};

#[test]
fn test_normal_and_asm() {
    env_logger::init();
    use super::entity::Fr;
    use assembly::{
        add as asm_add, double as asm_double, mul as asm_mul, neg as asm_neg, square as asm_square,
        sub as asm_sub,
    };
    use normal::{add, double, mul, neg, square, sub};
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    for i in 0..1000 {
        let mut a_seeds = [
            0x43, 0x62, 0xbe, 0x7d, 0x23, 0xad, 0x56, 0xcd, 0x33, 0x0a, 0x22, 0x23, 0x46, 0x36,
            0xac, 0xef,
        ];
        let mut b_seeds = [
            0xef, 0xac, 0x36, 0x7d, 0x23, 0x23, 0x22, 0x0a, 0x33, 0xcd, 0x56, 0xad, 0x46, 0xbe,
            0x62, 0x43,
        ];
        let seed = i as u8 % u8::MAX;
        let index = (seed % 16) as usize;
        a_seeds[index] = seed;
        b_seeds[index] = seed;
        if i % 2 != 0 {
            a_seeds.reverse();
        } else {
            b_seeds.reverse();
        }
        let a_rng = XorShiftRng::from_seed(a_seeds);
        let b_rng = XorShiftRng::from_seed(b_seeds);

        let a = Fr::random(a_rng);
        let b = Fr::random(b_rng);

        log::debug!("i = {i}");
        log::debug!("a = {:?}", a);
        log::debug!("b = {:?}", b);

        assert_eq!(add(&a.0, &b.0), asm_add(&a.0, &b.0));
        assert_eq!(sub(&a.0, &b.0), asm_sub(&a.0, &b.0));
        assert_eq!(double(&a.0), asm_double(&a.0));
        assert_eq!(neg(&a.0), asm_neg(&a.0));
        assert_eq!(square(&a.0), asm_square(&a.0));
        assert_eq!(mul(&a.0, &b.0), asm_mul(&a.0, &b.0));
    }
}
