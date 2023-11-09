use crate::{matrix::SparseRow, R1cs};

use zkstd::common::{vec, PrimeField};

pub struct FieldAssignment<F: PrimeField>(pub SparseRow<F>);

impl<F: PrimeField> FieldAssignment<F> {
    pub fn instance(cs: &mut R1cs<F>, instance: F) -> SparseRow<F> {
        let wire = cs.public_wire();
        cs.x.push(instance);

        SparseRow(vec![(wire, F::one())])
    }

    pub fn witness(cs: &mut R1cs<F>, witness: F) -> SparseRow<F> {
        let wire = cs.private_wire();
        cs.w.push(witness);

        SparseRow(vec![(wire, F::one())])
    }

    pub fn mul(cs: &mut R1cs<F>, x: &SparseRow<F>, y: &SparseRow<F>) -> SparseRow<F> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let witness = x.evaluate(&cs.x, &cs.w) * y.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.mul_gate(x, y, &z);

        z
    }

    pub fn add(cs: &mut R1cs<F>, x: &SparseRow<F>, y: &SparseRow<F>) -> SparseRow<F> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let witness = x.evaluate(&cs.x, &cs.w) + y.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.add_gate(x, y, &z);

        z
    }

    pub fn eq(cs: &mut R1cs<F>, x: &SparseRow<F>, y: &SparseRow<F>) {
        cs.mul_gate(x, &SparseRow::one(), y)
    }
}
