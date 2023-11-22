use crate::{
    pedersen::PedersenCommitment,
    relaxed_r1cs::{RelaxedR1cs, RelaxedR1csInstance, RelaxedR1csWitness},
};

use crate::hash::MimcRO;
use r1cs::{CircuitDriver, DenseVectors, R1cs};
use zkstd::common::{Ring, RngCore};

pub struct Prover<C: CircuitDriver> {
    // public parameters
    pp: PedersenCommitment<C::Affine>,

    // r1cs structure
    f: R1cs<C>,
}

impl<C: CircuitDriver> Prover<C> {
    pub fn new(f: R1cs<C>, rng: impl RngCore) -> Self {
        let m = f.m();
        let n = m.next_power_of_two() as u64;
        let pp = PedersenCommitment::new(n, rng);

        Self { pp, f }
    }

    pub fn prove(
        &self,
        r1cs: &R1cs<C>,
        relaxed_r1cs: &RelaxedR1cs<C>,
    ) -> (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>, C::Affine) {
        let mut transcript = MimcRO::<10, C::Base>::default();
        // compute cross term t
        let t = self.compute_cross_term(r1cs, relaxed_r1cs);

        let commit_t = self.pp.commit(&t);

        transcript.append_point(commit_t);
        relaxed_r1cs.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze().into();

        // fold instance
        let instance = relaxed_r1cs.fold_instance(r1cs, r, commit_t);

        // fold witness
        let witness = relaxed_r1cs.fold_witness(r1cs, r, t);

        // return folded relaxed r1cs instance, witness and commit T
        (instance, witness, commit_t)
    }

    // T = AZ1 ◦ BZ2 + AZ2 ◦ BZ1 − u1 · CZ2 − u2 · CZ1
    fn compute_cross_term(
        &self,
        r1cs: &R1cs<C>,
        relaxed_r1cs: &RelaxedR1cs<C>,
    ) -> DenseVectors<C::Scalar> {
        let u1 = C::Scalar::one();
        let u2 = relaxed_r1cs.u();
        let m = self.f.m();
        let (a, b, c) = self.f.matrices();
        let (w0, w1) = (DenseVectors::new(r1cs.w()), relaxed_r1cs.w());
        let (x0, x1) = (DenseVectors::new(r1cs.x()), relaxed_r1cs.x());

        // matrices and z vector matrix multiplication
        let az2 = a.prod(&m, &x1, &w1);
        let bz1 = b.prod(&m, &x0, &w0);
        let az1 = a.prod(&m, &x0, &w0);
        let bz2 = b.prod(&m, &x1, &w1);
        let cz2 = c.prod(&m, &x1, &w1);
        let cz1 = c.prod(&m, &x0, &w0);

        // matrices Hadamard product
        let az2bz1 = az2 * bz1;
        let az1bz2 = az1 * bz2;

        // vector scalar mutltiplication
        let c1cz2 = cz2 * u1;
        let c2cz1 = cz1 * u2;

        // vector addition and subtraction
        az2bz1 + az1bz2 - c1cz2 - c2cz1
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Prover, RelaxedR1cs};

    use r1cs::{test::example_r1cs, GrumpkinDriver};
    use zkstd::common::OsRng;

    pub(crate) fn example_prover() -> Prover<GrumpkinDriver> {
        let r1cs = example_r1cs(0);
        Prover::new(r1cs, OsRng)
    }

    #[test]
    fn folding_scheme_prover_test() {
        let prover = example_prover();
        let r1cs = example_r1cs(1);
        let mut relaxed_r1cs = RelaxedR1cs::new(r1cs);
        for i in 1..10 {
            let r1cs = example_r1cs(i);
            let (instance, witness, _) = prover.prove(&r1cs, &relaxed_r1cs);
            relaxed_r1cs = relaxed_r1cs.update(&instance, &witness);
        }

        assert!(relaxed_r1cs.is_sat())
    }
}
