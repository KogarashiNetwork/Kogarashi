mod instance;
mod witness;

use crate::PedersenCommitment;
pub(crate) use instance::{R1csInstance, RelaxedR1csInstance};
pub(crate) use witness::{R1csWitness, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Decode, Encode, Ring};
use zkstd::matrix::{DenseVectors, SparseMatrix};

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct R1csShape<C: CircuitDriver> {
    #[codec(compact)]
    m: u64,
    #[codec(compact)]
    instance_length: u64,
    #[codec(compact)]
    witness_length: u64,
    a: SparseMatrix<C::Scalar>,
    b: SparseMatrix<C::Scalar>,
    c: SparseMatrix<C::Scalar>,
}

pub(crate) fn r1cs_instance_and_witness<C: CircuitDriver>(
    cs: &R1cs<C>,
    shape: &R1csShape<C>,
    ck: &PedersenCommitment<C::Affine>,
) -> (R1csInstance<C>, R1csWitness<C>) {
    assert_eq!(cs.m_l_1(), shape.m_l_1() as usize);
    assert_eq!(cs.m(), shape.m());
    let w = cs.w();
    let x = cs.x()[1..].to_vec();
    assert_eq!(x.len(), shape.l() as usize);

    let witness = R1csWitness::new(shape, w);
    let commit_w = witness.commit(ck);
    let instance = R1csInstance::new(shape, commit_w, x);

    (instance, witness)
}

impl<C: CircuitDriver> From<R1cs<C>> for R1csShape<C> {
    fn from(value: R1cs<C>) -> Self {
        let (a, b, c) = value.matrices();
        Self {
            m: value.m(),
            instance_length: (value.l() - 1) as u64,
            witness_length: value.m_l_1() as u64,
            a,
            b,
            c,
        }
    }
}

impl<C: CircuitDriver> R1csShape<C> {
    #[allow(clippy::type_complexity)]
    pub fn matrices(
        &self,
    ) -> (
        SparseMatrix<C::Scalar>,
        SparseMatrix<C::Scalar>,
        SparseMatrix<C::Scalar>,
    ) {
        (self.a.clone(), self.b.clone(), self.c.clone())
    }

    pub fn m(&self) -> u64 {
        self.m
    }

    pub fn l(&self) -> u64 {
        self.instance_length
    }

    pub fn m_l_1(&self) -> u64 {
        self.witness_length
    }

    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub fn is_sat_relaxed(
        &self,
        instance: &RelaxedR1csInstance<C>,
        witness: &RelaxedR1csWitness<C>,
    ) -> bool {
        let Self { m, a, b, c, .. } = self;

        let RelaxedR1csInstance {
            commit_w: _,
            commit_e: _,
            u,
            x,
        } = instance;
        let RelaxedR1csWitness { w, e } = witness;

        let l = x.len() + 1;
        let z = DenseVectors::new(vec![vec![*u], x.get(), w.get()].concat());
        // A · Z
        let az = a.prod(*m, l, &z);
        // B · Z
        let bz = b.prod(*m, l, &z);
        // C · Z
        let cz = c.prod(*m, l, &z);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        // u · (C · Z) + E
        let ucz = cz * *u;
        let ucze = ucz + e.clone();

        azbz.iter()
            .zip(ucze.iter())
            .all(|(left, right)| left == right)
    }

    ///  check (A · Z) ◦ (B · Z) = (C · Z)
    pub fn is_sat(
        &self,
        ck: &PedersenCommitment<C::Affine>,
        instance: &R1csInstance<C>,
        witness: &R1csWitness<C>,
    ) -> bool {
        let Self { m, a, b, c, .. } = self;

        let R1csInstance { commit_w, x } = instance;
        let R1csWitness { w } = witness;

        let l = x.len() + 1;
        let z = DenseVectors::new(vec![vec![C::Scalar::one()], x.get(), w.get()].concat());
        // A · Z
        let az = a.prod(*m, l, &z);
        // B · Z
        let bz = b.prod(*m, l, &z);
        // C · Z
        let cz = c.prod(*m, l, &z);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        let constraints_check = azbz
            .iter()
            .zip(cz.iter())
            .all(|(left, right)| left == right);
        let commit_check = *commit_w == witness.commit(ck);

        constraints_check && commit_check
    }
}

#[cfg(test)]
mod tests {
    use super::{r1cs_instance_and_witness, R1csShape, RelaxedR1csInstance, RelaxedR1csWitness};

    use grumpkin::Affine;
    use rand_core::OsRng;

    use crate::driver::GrumpkinDriver;
    use crate::PedersenCommitment;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn relaxed_r1cs_test() {
        let mut rng = OsRng;
        for i in 1..10 {
            let r1cs: R1cs<GrumpkinDriver> = example_r1cs(i);
            let shape = R1csShape::from(r1cs.clone());
            let k = shape.m().next_power_of_two().trailing_zeros();
            let ck = PedersenCommitment::<Affine>::new(k.into(), &mut rng);
            let (x, w) = r1cs_instance_and_witness(&r1cs, &shape, &ck);
            let instance = RelaxedR1csInstance::from_r1cs_instance(&ck, &shape, &x);
            let witness = RelaxedR1csWitness::from_r1cs_witness(&shape, &w);
            assert!(shape.is_sat_relaxed(&instance, &witness))
        }
    }
}
