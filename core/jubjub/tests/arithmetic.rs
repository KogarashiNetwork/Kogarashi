use zero_jubjub::Fr;

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    #[test]
    fn add_test() {
        for i in 0..1000000 {
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

            // a + a = a * 2
            a.add_assign(b);
            c.double_assign();

            assert_eq!(a, c);
        }
    }

    #[test]
    fn sub_test() {
        for i in 0..1000000 {
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
            let mut d = a.clone();

            // a - a = a * 2 - a * 2
            a.sub_assign(b);
            c.double_assign();

            d.double_assign();
            c.sub_assign(d);

            assert_eq!(a, c);
        }
    }

    #[test]
    fn mul_test() {
        for i in 0..1000000 {
            let mut a_seeds = [
                0x43, 0x62, 0xbe, 0x7d, 0x23, 0xad, 0x56, 0xcd, 0x33, 0x0a, 0x22, 0x23, 0x46, 0x36,
                0xac, 0xef,
            ];
            let mut b_seeds = [
                0xef, 0xac, 0x36, 0x7d, 0x23, 0x23, 0x22, 0x0a, 0x33, 0xcd, 0x56, 0xad, 0x46, 0xbe,
                0x62, 0x43,
            ];
            let mut c_seeds = [
                0x5d, 0xbe, 0x62, 0x59, 0x8d, 0x31, 0x3d, 0x76, 0x32, 0x37, 0xdb, 0x17, 0xe5, 0xbc,
                0x06, 0x54,
            ];
            let seed = i as u8 % u8::MAX;
            let index = (seed % 16) as usize;
            a_seeds[index] = seed;
            b_seeds[index] = seed;
            c_seeds[index] = seed;
            if i % 3 != 0 {
                a_seeds.reverse();
            } else if i % 3 != 1 {
                b_seeds.reverse();
            } else {
                c_seeds.reverse();
            }
            let a_rng = XorShiftRng::from_seed(a_seeds);
            let b_rng = XorShiftRng::from_seed(b_seeds);
            let c_rng = XorShiftRng::from_seed(c_seeds);

            let mut a = Fr::random(a_rng);
            let b = Fr::random(b_rng);
            let c = Fr::random(c_rng);
            let mut a2 = a.clone();
            let mut b2 = b.clone();
            let c2 = c.clone();
            let mut a3 = a.clone();

            // a * b + a * c
            a.mul_assign(b);
            a2.mul_assign(c);
            a.add_assign(a2);

            // a * (b + c)
            b2.add_assign(c2);
            a3.mul_assign(b2);

            assert_eq!(a, a3);
        }
    }
}
