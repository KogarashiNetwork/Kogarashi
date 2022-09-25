use core::ops::Mul;
use parity_scale_codec::alloc::vec::Vec;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use zero_jubjub::{
    arithmetic::poly::Polynomial, coordinate::Projective, fr::Fr, interface::Coordinate,
};

pub struct Kzg {
    k: u32,
    n: u64,
    g1_generator: Projective,
    g1_projective: Vec<Projective>,
    g2_generator: Projective,
    g2_projective: Projective,
}

pub struct Witness {
    poly_commit: Projective,
    q_poly_commit: Projective,
    poly_at_commit: Projective,
}

impl Kzg {
    pub fn setup(k: u32) -> Self {
        let n = 1 << k;
        let lambda = generate_security_param();
        let (g1_generator, g2_generator) = (Projective::g1(), Projective::g2());

        let mut acc = Fr::one();
        let mut g1_projective = Vec::new();
        let g2_projective = lambda * g2_generator.clone();

        for _ in 0..n {
            g1_projective.push(acc * g1_generator.clone());
            acc *= lambda;
        }

        Kzg {
            k,
            n,
            g1_generator,
            g1_projective,
            g2_generator,
            g2_projective,
        }
    }

    fn commit(self, poly: &Polynomial) -> Projective {
        assert_eq!(self.n, poly.0.len() as u64);

        poly.0.iter().rev().zip(self.g1_projective.iter()).fold(
            Projective::identity(),
            |mut acc, (coeff, at)| {
                acc.add(at.clone().mul(*coeff));
                acc
            },
        )
    }

    fn create_witness(self, poly: Polynomial, at: Fr) -> Witness {
        let poly_commit = self.commit(&poly);
        let poly_at_commit = poly.evaluate(at);
        // Todo
        // vanish poly
        // commit quotient poly
        let q_poly_commit = Projective::identity();

        Witness {
            poly_commit,
            q_poly_commit,
            poly_at_commit,
        }
    }
}

fn generate_security_param() -> Fr {
    let rng = XorShiftRng::from_seed([
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
    ]);
    Fr::random(rng)
}
