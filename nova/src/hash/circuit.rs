use crate::hash::Mimc;
use r1cs::gadget::curve::PointAssignment;
use r1cs::gadget::field::FieldAssignment;
use r1cs::{CircuitDriver, R1cs};
use zkstd::common::IntGroup;

pub(crate) struct ZKMimc<const ROUND: usize, C: CircuitDriver> {
    constants: [C::Scalar; ROUND],
}

impl<const ROUND: usize, C: CircuitDriver> Default for ZKMimc<ROUND, C> {
    fn default() -> Self {
        Self {
            constants: Mimc::<ROUND, C::Scalar>::default().constants,
        }
    }
}

impl<const ROUND: usize, C: CircuitDriver> ZKMimc<ROUND, C> {
    pub(crate) fn hash(
        &self,
        cs: &mut R1cs<C>,
        mut xl: FieldAssignment<C>,
        mut xr: FieldAssignment<C>,
    ) -> FieldAssignment<C> {
        for c in self.constants.iter().map(|c| FieldAssignment::constant(c)) {
            let cxl = &xl + &c;
            let mut ccxl = FieldAssignment::square(cs, &cxl);
            ccxl = &FieldAssignment::mul(cs, &ccxl, &cxl) + &xr;
            xr = xl;
            xl = ccxl;
        }
        xl
    }
}

pub(crate) struct MimcROCircuit<const ROUND: usize, C: CircuitDriver> {
    hasher: ZKMimc<ROUND, C>,
    state: Vec<FieldAssignment<C>>,
    key: FieldAssignment<C>,
}

impl<const ROUND: usize, C: CircuitDriver> Default for MimcROCircuit<ROUND, C> {
    fn default() -> Self {
        Self {
            hasher: ZKMimc::default(),
            state: Vec::default(),
            key: FieldAssignment::constant(&C::Scalar::zero()),
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
    use crate::hash::circuit::MimcROCircuit;
    use crate::hash::MimcRO;
    use bn_254::Fr;
    use grumpkin::Affine;
    use r1cs::gadget::curve::PointAssignment;
    use r1cs::gadget::field::FieldAssignment;
    use r1cs::{GrumpkinDriver, R1cs};
    use rand_core::OsRng;
    use zkstd::common::{CurveGroup, Group};

    #[test]
    fn mimc_circuit() {
        let mut mimc = MimcRO::<322, Fr>::default();
        let mut mimc_circuit = MimcROCircuit::<322, GrumpkinDriver>::default();
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let point = Affine::random(OsRng);
        let scalar = Fr::random(OsRng);

        let point_assignment =
            PointAssignment::instance(&mut cs, point.get_x(), point.get_y(), point.is_identity());
        let scalar_assignment = FieldAssignment::instance(&mut cs, scalar);
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
