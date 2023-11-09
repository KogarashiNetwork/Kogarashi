use crate::wire::Wire;
use crate::{matrix::SparseRow, R1cs};

use zkstd::common::{vec, Add, PrimeField};

pub struct FieldAssignment<F: PrimeField>(SparseRow<F>);

impl<F: PrimeField> FieldAssignment<F> {
    pub fn instance(cs: &mut R1cs<F>, instance: F) -> Self {
        let wire = cs.public_wire();
        cs.x.push(instance);

        Self(SparseRow(vec![(wire, F::one())]))
    }

    pub fn witness(cs: &mut R1cs<F>, witness: F) -> Self {
        let wire = cs.private_wire();
        cs.w.push(witness);

        Self(SparseRow(vec![(wire, F::one())]))
    }

    pub fn constant(constant: F) -> Self {
        Self(SparseRow(vec![(Wire::Instance(0), constant)]))
    }

    pub fn mul(cs: &mut R1cs<F>, x: &Self, y: &Self) -> Self {
        if let Some(c) = x.0.as_constant() {
            return Self(y.0.clone() * c);
        }
        if let Some(c) = y.0.as_constant() {
            return Self(x.0.clone() * c);
        }

        let witness = x.0.evaluate(&cs.x, &cs.w) * y.0.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.mul_gate(&x.0, &y.0, &z.0);

        z
    }

    pub fn add(cs: &mut R1cs<F>, x: &Self, y: &Self) -> Self {
        if let Some(c) = x.0.as_constant() {
            return Self(y.0.clone() * c);
        }
        if let Some(c) = y.0.as_constant() {
            return Self(x.0.clone() * c);
        }

        let witness = x.0.evaluate(&cs.x, &cs.w) + y.0.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.add_gate(&x.0, &y.0, &z.0);

        z
    }

    pub fn eq(cs: &mut R1cs<F>, x: &Self, y: &Self) {
        cs.mul_gate(&x.0, &SparseRow::one(), &y.0)
    }
}

impl<F: PrimeField> Add for FieldAssignment<F> {
    type Output = FieldAssignment<F>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldAssignment, R1cs};
    use jub_jub::Fr as Scalar;
    use zkstd::common::{Group, OsRng};

    #[test]
    fn field_add_test() {
        let a = Scalar::random(OsRng);
        let b = Scalar::random(OsRng);
        let mut c = a + b;

        let mut cs = R1cs::default();
        let mut ncs = cs.clone();

        // a + b == c
        let x = FieldAssignment::instance(&mut cs, a);
        let y = FieldAssignment::witness(&mut cs, b);
        let z = FieldAssignment::instance(&mut cs, c);
        let sum = FieldAssignment::add(&mut cs, &x, &y);
        FieldAssignment::eq(&mut cs, &z, &sum);

        // a + b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let sum = FieldAssignment::add(&mut ncs, &x, &y);
        FieldAssignment::eq(&mut ncs, &z, &sum);

        assert!(!ncs.is_sat())
    }
}
