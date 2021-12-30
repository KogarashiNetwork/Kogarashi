use zero_jubjub::Fr;

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    #[test]
    fn add_test() {
        for i in 0..100000 {
            let mut initial_seeds = [
                0x43, 0x62, 0xbe, 0x7d, 0x23, 0xad, 0x56, 0xcd, 0x33, 0x0a, 0x22, 0x23, 0x46, 0x36,
                0xac, 0xef,
            ];
            let seed = i as u8 % u8::MAX;
            let index = (seed % 16) as usize;
            initial_seeds[index] = seed;
            let rng = XorShiftRng::from_seed(initial_seeds);

            let mut a = Fr::random(rng);
            let b = a.clone();
            let mut c = a.clone();
            a.add_assign(b);
            c.double_assign();
            assert_eq!(a, c);
        }
    }
}
