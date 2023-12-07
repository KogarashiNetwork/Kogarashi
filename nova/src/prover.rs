use crate::{
    pedersen::PedersenCommitment,
    relaxed_r1cs::{RelaxedR1cs, RelaxedR1csInstance, RelaxedR1csWitness},
};

use crate::hash::{MimcRO, MIMC_ROUNDS};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Ring, RngCore};
use zkstd::matrix::DenseVectors;

pub struct Prover<C: CircuitDriver> {
    // public parameters
    pub(crate) pp: PedersenCommitment<C::Affine>,

    // r1cs structure
    f: R1cs<C>,
}

impl<C: CircuitDriver> Prover<C> {
    pub fn new(f: R1cs<C>, rng: impl RngCore) -> Self {
        let m = f.m();
        let n = m.next_power_of_two() as u64;
        let k = n.trailing_zeros();

        let pp = PedersenCommitment::new(k.into(), rng);

        Self { pp, f }
    }

    pub fn prove(
        &self,
        r1cs_1: &RelaxedR1cs<C>,
        r1cs_2: &RelaxedR1cs<C>,
    ) -> (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>, C::Affine) {
        let mut transcript = MimcRO::<MIMC_ROUNDS, C::Base>::default();
        // compute cross term t
        let t = self.compute_cross_term(r1cs_1, r1cs_2);

        let commit_t = self.pp.commit(&t);

        transcript.append_point(commit_t);
        r1cs_2.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze().into();

        // fold instance
        let instance = r1cs_2.fold_instance(r1cs_1, r, commit_t);

        // fold witness
        let witness = r1cs_2.fold_witness(r1cs_1, r, t);

        // return folded relaxed r1cs instance, witness and commit T
        (instance, witness, commit_t)
    }

    // T = AZ1 ◦ BZ2 + AZ2 ◦ BZ1 − u1 · CZ2 − u2 · CZ1
    pub(crate) fn compute_cross_term(
        &self,
        r1cs_1: &RelaxedR1cs<C>,
        r1cs_2: &RelaxedR1cs<C>,
    ) -> DenseVectors<C::Scalar> {
        let u1 = C::Scalar::one();
        let u2 = r1cs_2.u();
        let m = self.f.m();
        let (a, b, c) = self.f.matrices();
        let (w0, w1) = (r1cs_1.w(), r1cs_2.w());
        let (x0, x1) = (r1cs_1.x(), r1cs_2.x());

        let z1 = DenseVectors::new(vec![vec![u1], x0.get(), w0.get()].concat());
        let l1 = x0.len() + 1;
        let z2 = DenseVectors::new(vec![vec![u2], x1.get(), w1.get()].concat());
        let l2 = x1.len() + 1;

        // matrices and z vector matrix multiplication
        let az2 = a.prod(&m, l2, &z2);
        let bz1 = b.prod(&m, l1, &z1);
        let az1 = a.prod(&m, l1, &z1);
        let bz2 = b.prod(&m, l2, &z2);
        let cz2 = c.prod(&m, l2, &z2);
        let cz1 = c.prod(&m, l1, &z1);

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
    use bn_254::{Fq, Fr};

    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::Verifier;
    use grumpkin::driver::GrumpkinDriver;
    use zkstd::common::OsRng;
    use zkstd::r1cs::test::example_r1cs;

    pub(crate) fn example_prover() -> Prover<GrumpkinDriver> {
        let r1cs = example_r1cs(0);
        Prover::new(r1cs, OsRng)
    }

    #[test]
    fn nifs_folding_test() {
        let prover = example_prover();

        let mut transcript = MimcRO::<MIMC_ROUNDS, Fq>::default();
        let r1cs_1 = example_r1cs(4);
        let r1cs_2 = example_r1cs(3);

        let relaxed_r1cs_1 = RelaxedR1cs::new(r1cs_1);
        let mut relaxed_r1cs_2 = RelaxedR1cs::new(r1cs_2);

        let (folded_instance, witness, commit_t) = prover.prove(&relaxed_r1cs_1, &relaxed_r1cs_2);
        let verified_instance = Verifier::verify(commit_t, &relaxed_r1cs_1, &relaxed_r1cs_2);
        assert_eq!(folded_instance, verified_instance);

        transcript.append_point(commit_t);
        relaxed_r1cs_2.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(&relaxed_r1cs_1, &relaxed_r1cs_2);
        let r = Fr::from(transcript.squeeze());

        // naive check that the folded witness satisfies the relaxed r1cs
        let z3: Vec<Fr> = [
            vec![verified_instance.u],
            verified_instance.x.get(),
            witness.w.get(),
        ]
        .concat();

        let z1 = [
            vec![Fr::one()],
            relaxed_r1cs_1.x().get(),
            relaxed_r1cs_1.w().get(),
        ]
        .concat();
        let z2 = [
            vec![relaxed_r1cs_2.instance.u],
            relaxed_r1cs_2.x().get(),
            relaxed_r1cs_2.w().get(),
        ]
        .concat();

        let z3_aux: Vec<Fr> = z2
            .iter()
            .map(|x| x * r)
            .zip(z1)
            .map(|(x, y)| x + y)
            .collect();

        assert_eq!(z3, z3_aux);

        // check that relations hold for the 2 inputted instances and the folded one
        let instance1 = relaxed_r1cs_1.instance.clone();
        let instance2 = relaxed_r1cs_2.instance.clone();
        assert!(relaxed_r1cs_2.is_sat());
        relaxed_r1cs_2 = relaxed_r1cs_2.update(&relaxed_r1cs_1.instance, &relaxed_r1cs_1.witness);
        assert!(relaxed_r1cs_2.is_sat());
        relaxed_r1cs_2 = relaxed_r1cs_2.update(&folded_instance, &witness);
        assert!(relaxed_r1cs_2.is_sat());

        // next equalities should hold since we started from two cmE of zero-vector E's
        assert_eq!(verified_instance.commit_e, (commit_t * r).into());
        assert_eq!(witness.e, t * r);

        let r2 = r * r;
        assert!(
            folded_instance.commit_e
                == (instance1.commit_e + commit_t * r + instance2.commit_e * r2).into()
                && folded_instance.u == instance1.u + r * instance2.u
                && folded_instance.commit_w == (instance1.commit_w + instance2.commit_w * r).into()
                && folded_instance.x == &instance1.x + &(&instance2.x * r)
        );
    }
}
