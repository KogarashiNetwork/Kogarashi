use crate::hash::Mimc;

use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, R1cs};
use zkstd::common::PrimeField;

pub(crate) struct MimcAssignment<const ROUND: usize, F: PrimeField> {
    constants: [F; ROUND],
}

impl<const ROUND: usize, F: PrimeField> Default for MimcAssignment<ROUND, F> {
    fn default() -> Self {
        Self {
            constants: Mimc::<ROUND, F>::default().constants,
        }
    }
}

impl<const ROUND: usize, F: PrimeField> MimcAssignment<ROUND, F> {
    pub(crate) fn hash<C: CircuitDriver<Base = F>>(
        &self,
        cs: &mut R1cs<C>,
        mut xl: FieldAssignment<F>,
        mut xr: FieldAssignment<F>,
    ) -> FieldAssignment<F> {
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
