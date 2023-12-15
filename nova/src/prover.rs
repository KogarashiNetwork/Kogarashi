use crate::{
    pedersen::PedersenCommitment,
    relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness},
};

use crate::hash::{MimcRO, MIMC_ROUNDS};
use crate::relaxed_r1cs::{R1csInstance, R1csShape, R1csWitness};
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::Ring;
use zkstd::matrix::DenseVectors;

pub struct Prover<C: CircuitDriver> {
    // public parameters
    pub(crate) ck: PedersenCommitment<C::Affine>,

    // r1cs structure
    f: R1csShape<C>,
}

impl<C: CircuitDriver> Prover<C> {
    pub fn new(shape: R1csShape<C>, ck: PedersenCommitment<C::Affine>) -> Self {
        Self { ck, f: shape }
    }

    pub fn prove(
        &self,
        instance1: &RelaxedR1csInstance<C>,
        witness1: &RelaxedR1csWitness<C>,
        instance2: &R1csInstance<C>,
        witness2: &R1csWitness<C>,
    ) -> (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>, C::Affine) {
        let mut transcript = MimcRO::<MIMC_ROUNDS, C>::default();
        // compute cross term t
        let t = self.compute_cross_term(instance1, witness1, instance2, witness2);

        let commit_t = self.ck.commit(&t);

        transcript.append_point(commit_t);
        instance1.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze();

        dbg!(r);

        // fold instance
        let instance = instance1.fold(instance2, r, commit_t);

        // fold witness
        let witness = witness1.fold(witness2, r, t);

        // return folded relaxed r1cs instance, witness and commit T
        (instance, witness, commit_t)
    }

    // T = AZ1 ◦ BZ2 + AZ2 ◦ BZ1 − u1 · CZ2 − u2 · CZ1
    pub(crate) fn compute_cross_term(
        &self,
        instance1: &RelaxedR1csInstance<C>,
        witness1: &RelaxedR1csWitness<C>,
        instance2: &R1csInstance<C>,
        witness2: &R1csWitness<C>,
    ) -> DenseVectors<C::Scalar> {
        let u1 = instance1.u;
        let u2 = C::Scalar::one();
        let m = self.f.m();
        let (a, b, c) = self.f.matrices();
        let (w1, w2) = (witness1.w(), witness2.w());
        let (x1, x2) = (instance1.x(), instance2.x());

        let z1 = DenseVectors::new(vec![vec![u1], x1.get(), w1.get()].concat());
        let l1 = x1.len() + 1;
        let z2 = DenseVectors::new(vec![vec![u2], x2.get(), w2.get()].concat());
        let l2 = x2.len() + 1;

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
    use super::Prover;
    use bn_254::Fq;
    use zkstd::circuit::CircuitDriver;

    use crate::driver::GrumpkinDriver;
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::relaxed_r1cs::{
        r1cs_instance_and_witness, R1csShape, RelaxedR1csInstance, RelaxedR1csWitness,
    };
    use crate::{PedersenCommitment, Verifier};
    use zkstd::common::OsRng;
    use zkstd::r1cs::test::example_r1cs;

    pub(crate) fn example_prover<C: CircuitDriver>() -> Prover<C> {
        let r1cs = example_r1cs(0);
        let shape = R1csShape::from(r1cs);
        let k = (shape.m().next_power_of_two() as u64).trailing_zeros();
        let ck = PedersenCommitment::<C::Affine>::new(k.into(), OsRng);
        Prover::new(shape, ck)
    }

    #[test]
    fn nifs_folding_test() {
        let prover = example_prover();

        let mut transcript = MimcRO::<MIMC_ROUNDS, GrumpkinDriver>::default();
        let r1cs_1 = example_r1cs::<GrumpkinDriver>(4);
        let shape = R1csShape::from(r1cs_1.clone());
        let r1cs_2 = example_r1cs::<GrumpkinDriver>(3);

        let (x1, w1) = r1cs_instance_and_witness(&r1cs_1, &shape, &prover.ck);
        let instance1 = RelaxedR1csInstance::from_r1cs_instance(&prover.ck, &shape, &x1);
        let witness1 = RelaxedR1csWitness::from_r1cs_witness(&shape, &w1);

        let (instance2, witness2) = r1cs_instance_and_witness(&r1cs_2, &shape, &prover.ck);

        let (folded_instance, folded_witness, commit_t) =
            prover.prove(&instance1, &witness1, &instance2, &witness2);
        let verified_instance = Verifier::verify(commit_t, &instance1, &instance2);
        assert_eq!(folded_instance, verified_instance);

        transcript.append_point(commit_t);
        instance1.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(&instance1, &witness1, &instance2, &witness2);
        let r = transcript.squeeze();

        // naive check that the folded witness satisfies the relaxed r1cs
        let z3: Vec<Fq> = [
            vec![verified_instance.u],
            verified_instance.x.get(),
            folded_witness.w.get(),
        ]
        .concat();

        let z1 = [vec![instance1.u()], instance1.x().get(), witness1.w().get()].concat();
        let z2 = [vec![Fq::one()], instance2.x().get(), witness2.w().get()].concat();

        let z3_aux: Vec<Fq> = z2
            .iter()
            .map(|x| x * r)
            .zip(z1)
            .map(|(x, y)| x + y)
            .collect();

        assert_eq!(z3, z3_aux);

        // check that relations hold for the 2 inputted instances and the folded one
        assert!(shape.is_sat_relaxed(&instance1, &witness1));
        assert!(shape.is_sat(&prover.ck, &instance2, &witness2));
        assert!(shape.is_sat_relaxed(&folded_instance, &folded_witness));

        // next equalities should hold since we started from two cmE of zero-vector E's
        assert_eq!(verified_instance.commit_e, (commit_t * r).into());
        assert_eq!(folded_witness.e, t * r);

        let r2 = r * r;
        assert!(
            folded_instance.commit_e == (instance1.commit_e + commit_t * r).into()
                && folded_instance.u == instance1.u + r
                && folded_instance.commit_w == (instance1.commit_w + instance2.commit_w * r).into()
                && folded_instance.x == &instance1.x + &(&instance2.x * r)
        );
    }
}
