use parity_scale_codec::alloc::vec::Vec;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_jubjub::{coordinate::Projective, fr::Fr};

pub struct Kzg {
    k: u32,
    g1_projective: Vec<Projective>,
    g2_projective: Vec<Projective>,
}

impl Kzg {
    pub fn setup(k: u32) -> Self {
        let n = 1 << k;
        let lambda = generate_security_param();
        let mut acc = Fr::one();
        let mut g1_projective = Vec::new();
        let mut g2_projective = Vec::new();

        let (g1, g2) = (Projective::g1(), Projective::g2());

        for _ in 0..n {
            g1_projective.push(acc * g1.clone());
            g2_projective.push(acc * g2.clone());
            acc *= lambda;
        }

        Kzg {
            k,
            g1_projective,
            g2_projective,
        }
    }
}

fn generate_security_param() -> Fr {
    let rng = XorShiftRng::from_seed([
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
    ]);
    Fr::random(rng)
}
