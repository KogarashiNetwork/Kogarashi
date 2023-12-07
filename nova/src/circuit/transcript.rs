use crate::gadget::MimcAssignment;

use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::IntGroup;

pub(crate) struct MimcROCircuit<const ROUND: usize, C: CircuitDriver> {
    hasher: MimcAssignment<ROUND, C>,
    state: Vec<FieldAssignment<C>>,
    key: FieldAssignment<C>,
}

impl<const ROUND: usize, C: CircuitDriver> Default for MimcROCircuit<ROUND, C> {
    fn default() -> Self {
        Self {
            hasher: MimcAssignment::default(),
            state: Vec::default(),
            key: FieldAssignment::constant(&C::Base::zero()),
        }
    }
}

impl<const ROUND: usize, C: CircuitDriver> MimcROCircuit<ROUND, C> {
    pub(crate) fn append(&mut self, absorb: FieldAssignment<C>) {
        self.state.push(absorb)
    }

    pub(crate) fn append_point(&mut self, point: PointAssignment<C>) {
        self.append(point.get_x());
        self.append(point.get_y());
        self.append(point.get_z());
    }

    pub(crate) fn squeeze(&self, cs: &mut R1cs<C>) -> FieldAssignment<C> {
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

    use bn_254::Fr;
    use grumpkin::{driver::GrumpkinDriver, Affine};
    use rand_core::OsRng;
    use zkstd::circuit::prelude::{FieldAssignment, PointAssignment, R1cs};
    use zkstd::common::Group;

    #[test]
    fn mimc_circuit() {
        let mut mimc = MimcRO::<MIMC_ROUNDS, Fr>::default();
        let mut mimc_circuit = MimcROCircuit::<MIMC_ROUNDS, GrumpkinDriver>::default();
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let point = Affine::random(OsRng);
        let scalar = Fr::random(OsRng);

        let point_assignment = PointAssignment::instance(&mut cs, point);
        let scalar_assignment = FieldAssignment::instance(&mut cs, scalar.into());
        mimc.append(scalar);
        mimc.append_point(point);
        mimc_circuit.append(scalar_assignment);
        mimc_circuit.append_point(point_assignment);

        let expected = mimc.squeeze();
        let circuit_result = mimc_circuit.squeeze(&mut cs);
        FieldAssignment::eq_constant(&mut cs, &circuit_result, &expected);
        assert!(cs.is_sat());
    }
}
