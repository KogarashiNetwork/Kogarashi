use crate::gadget::MimcAssignment;

use crate::hash::HASH_BITS;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::{IntGroup, Ring};

pub(crate) struct MimcROCircuit<const ROUND: usize, C: CircuitDriver> {
    hasher: MimcAssignment<ROUND, C::Base>,
    state: Vec<FieldAssignment<C::Base>>,
    key: FieldAssignment<C::Base>,
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
    pub(crate) fn append(&mut self, absorb: FieldAssignment<C::Base>) {
        self.state.push(absorb)
    }
    pub(crate) fn hash_vec<CS: CircuitDriver<Scalar = C::Base>>(
        &mut self,
        cs: &mut R1cs<CS>,
        values: Vec<FieldAssignment<C::Base>>,
    ) -> FieldAssignment<C::Base> {
        for x in values {
            self.state.push(x);
        }
        self.squeeze(cs, HASH_BITS)
    }

    pub(crate) fn append_point(&mut self, point: PointAssignment<C::Base>) {
        self.append(point.get_x());
        self.append(point.get_y());
        self.append(point.get_z());
    }

    pub(crate) fn squeeze<CS: CircuitDriver<Scalar = C::Base>>(
        &self,
        cs: &mut R1cs<CS>,
        num_bits: usize,
    ) -> FieldAssignment<C::Base> {
        let hash = self.state.iter().fold(self.key.clone(), |acc, scalar| {
            let h = self.hasher.hash(cs, scalar.clone(), acc.clone());
            &(&acc + scalar) + &h
        });

        let bits = FieldAssignment::to_bits(cs, &hash, num_bits);

        // TODO: Do faster
        let mut mult = FieldAssignment::constant(&C::Base::one());
        let mut val = FieldAssignment::constant(&C::Base::zero());
        for bit in bits.iter().rev().take(num_bits) {
            val = FieldAssignment::conditional_select(cs, &(&val + &mult), &val, bit);
            mult = &mult + &mult;
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::MimcROCircuit;
    use crate::hash::{MimcRO, HASH_BITS, MIMC_ROUNDS};

    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::Fr;
    use grumpkin::Affine;
    use rand_core::OsRng;
    use zkstd::circuit::prelude::{FieldAssignment, PointAssignment, R1cs};
    use zkstd::common::Group;

    #[test]
    fn mimc_circuit() {
        let mut mimc = MimcRO::<MIMC_ROUNDS, GrumpkinDriver>::default();
        let mut mimc_circuit = MimcROCircuit::<MIMC_ROUNDS, GrumpkinDriver>::default();
        let mut cs: R1cs<Bn254Driver> = R1cs::default();
        let point = Affine::random(OsRng);
        let scalar = Fr::random(OsRng);

        let point_assignment = PointAssignment::instance(&mut cs, point);
        let scalar_assignment = FieldAssignment::instance(&mut cs, scalar);
        mimc.append(scalar);
        mimc.append_point(point);
        mimc_circuit.append(scalar_assignment);
        mimc_circuit.append_point(point_assignment);

        let expected = mimc.squeeze(HASH_BITS).into();
        let circuit_result = mimc_circuit.squeeze(&mut cs, HASH_BITS);
        FieldAssignment::enforce_eq_constant(&mut cs, &circuit_result, &expected);
        assert!(cs.is_sat());
    }
}
