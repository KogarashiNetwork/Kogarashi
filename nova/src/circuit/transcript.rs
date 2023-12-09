use crate::gadget::MimcAssignment;

use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::{IntGroup, PrimeField};

pub(crate) struct MimcROCircuit<const ROUND: usize, F: PrimeField> {
    hasher: MimcAssignment<ROUND, F>,
    state: Vec<FieldAssignment<F>>,
    key: FieldAssignment<F>,
}

impl<const ROUND: usize, F: PrimeField> Default for MimcROCircuit<ROUND, F> {
    fn default() -> Self {
        Self {
            hasher: MimcAssignment::default(),
            state: Vec::default(),
            key: FieldAssignment::constant(&F::zero()),
        }
    }
}

impl<const ROUND: usize, F: PrimeField> MimcROCircuit<ROUND, F> {
    pub(crate) fn append(&mut self, absorb: FieldAssignment<F>) {
        self.state.push(absorb)
    }
    pub(crate) fn hash_vec<C: CircuitDriver<Base = F>>(
        &mut self,
        cs: &mut R1cs<C>,
        values: Vec<FieldAssignment<F>>,
    ) -> FieldAssignment<F> {
        for x in values {
            self.state.push(x);
        }
        self.squeeze(cs)
    }

    pub(crate) fn append_point(&mut self, point: PointAssignment<F>) {
        self.append(point.get_x());
        self.append(point.get_y());
        self.append(point.get_z());
    }

    pub(crate) fn squeeze<C: CircuitDriver<Base = F>>(
        &self,
        cs: &mut R1cs<C>,
    ) -> FieldAssignment<F> {
        self.state.iter().fold(self.key.clone(), |acc, scalar| {
            let h = self.hasher.hash(cs, scalar.clone(), acc.clone());
            &(&acc + scalar) + &h
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MimcROCircuit;
    use crate::hash::{MimcRO, MIMC_ROUNDS};

    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::{Fq, Fr, G1Affine};
    use rand_core::OsRng;
    use zkstd::circuit::prelude::{FieldAssignment, PointAssignment, R1cs};
    use zkstd::common::Group;

    #[test]
    fn mimc_circuit() {
        let mut mimc = MimcRO::<MIMC_ROUNDS, GrumpkinDriver>::default();
        let mut mimc_circuit = MimcROCircuit::<MIMC_ROUNDS, GrumpkinDriver>::default(); // Base = Fr, Scalar = Fq
        let mut cs: R1cs<Bn254Driver> = R1cs::default(); // Base = Fq, Scalar = Fr
        let point = G1Affine::random(OsRng); // Base = Fq, Scalar = Fr
        let scalar = Fr::random(OsRng);

        let point_assignment = PointAssignment::instance(&mut cs, point);
        let scalar_assignment = FieldAssignment::instance(&mut cs, scalar);
        mimc.append(scalar);
        mimc.append_point(point);
        mimc_circuit.append(scalar_assignment);
        mimc_circuit.append_point(point_assignment);

        let expected = mimc.squeeze().into();
        let circuit_result = mimc_circuit.squeeze(&mut cs);
        FieldAssignment::enforce_eq_constant(&mut cs, &circuit_result, &expected);
        assert!(cs.is_sat());
    }
}
