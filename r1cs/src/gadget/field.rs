use crate::circuit::CircuitDriver;
use crate::wire::Wire;
use crate::{matrix::SparseRow, R1cs};

use zkstd::common::{vec, Add, Ring};

pub struct FieldAssignment<C: CircuitDriver>(SparseRow<C::Base>);

impl<C: CircuitDriver> FieldAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, instance: C::Base) -> Self {
        let wire = cs.public_wire();
        cs.x.push(instance);

        Self(SparseRow(vec![(wire, C::Base::one())]))
    }

    pub fn witness(cs: &mut R1cs<C>, witness: C::Base) -> Self {
        let wire = cs.private_wire();
        cs.w.push(witness);

        Self(SparseRow(vec![(wire, C::Base::one())]))
    }

    pub fn constant(constant: &C::Base) -> Self {
        Self(SparseRow(vec![(Wire::Instance(0), *constant)]))
    }

    pub fn mul(cs: &mut R1cs<C>, x: &Self, y: &Self) -> Self {
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

    pub fn add(cs: &mut R1cs<C>, x: &Self, y: &Self) -> Self {
        if let Some(c) = x.0.as_constant() {
            return Self(y.0.clone() + SparseRow::from(c));
        }
        if let Some(c) = y.0.as_constant() {
            return Self(x.0.clone() + SparseRow::from(c));
        }

        let witness = x.0.evaluate(&cs.x, &cs.w) + y.0.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.add_gate(&x.0, &y.0, &z.0);

        z
    }

    pub fn sub(cs: &mut R1cs<C>, x: &Self, y: &Self) -> Self {
        if let Some(c) = x.0.as_constant() {
            return Self(y.0.clone() - SparseRow::from(c));
        }
        if let Some(c) = y.0.as_constant() {
            return Self(x.0.clone() - SparseRow::from(c));
        }

        let witness = x.0.evaluate(&cs.x, &cs.w) - y.0.evaluate(&cs.x, &cs.w);
        let z = Self::witness(cs, witness);
        cs.sub_gate(&x.0, &y.0, &z.0);

        z
    }

    pub fn eq(cs: &mut R1cs<C>, x: &Self, y: &Self) {
        cs.mul_gate(&x.0, &SparseRow::one(), &y.0)
    }
}

impl<C: CircuitDriver> Add for FieldAssignment<C> {
    type Output = FieldAssignment<C>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldAssignment, R1cs};
    use crate::test::GrumpkinDriver;
    use bn_254::Fr as Scalar;
    use zkstd::common::{Group, OsRng};

    #[test]
    fn field_add_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let a = Scalar::random(OsRng);
        let b = Scalar::random(OsRng);
        let mut c = a + b;

        // a + b == c
        let x = FieldAssignment::instance(&mut cs, a);
        let y = FieldAssignment::witness(&mut cs, b);
        let z = FieldAssignment::instance(&mut cs, c);
        let sum = FieldAssignment::add(&mut cs, &x, &y);
        FieldAssignment::eq(&mut cs, &z, &sum);

        assert!(cs.is_sat());

        // a + b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let sum = FieldAssignment::add(&mut ncs, &x, &y);
        FieldAssignment::eq(&mut ncs, &z, &sum);

        assert!(!ncs.is_sat())
    }

    #[test]
    fn field_mul_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let a = Scalar::random(OsRng);
        let b = Scalar::random(OsRng);
        let mut c = a * b;

        // a * b == c
        let x = FieldAssignment::instance(&mut cs, a);
        let y = FieldAssignment::witness(&mut cs, b);
        let z = FieldAssignment::instance(&mut cs, c);
        let product = FieldAssignment::mul(&mut cs, &x, &y);
        FieldAssignment::eq(&mut cs, &z, &product);

        assert!(cs.is_sat());

        // a * b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let product = FieldAssignment::mul(&mut ncs, &x, &y);
        FieldAssignment::eq(&mut ncs, &z, &product);

        assert!(!ncs.is_sat())
    }

    #[test]
    fn field_ops_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let input = Scalar::from(3);
        let c = Scalar::from(5);
        let out = Scalar::from(35);

        // x^3 + x + 5 == 35
        let x = FieldAssignment::witness(&mut cs, input);
        let c = FieldAssignment::constant(&c);
        let z = FieldAssignment::instance(&mut cs, out);
        let sym_1 = FieldAssignment::mul(&mut cs, &x, &x);
        let y = FieldAssignment::mul(&mut cs, &sym_1, &x);
        let sym_2 = FieldAssignment::add(&mut cs, &y, &x);
        FieldAssignment::eq(&mut cs, &z, &(sym_2 + c));

        assert!(cs.is_sat());

        // x^3 + x + 5 != 36
        let c = Scalar::from(5);
        let out = Scalar::from(36);
        let x = FieldAssignment::witness(&mut ncs, input);
        let c = FieldAssignment::constant(&c);
        let z = FieldAssignment::instance(&mut ncs, out);
        let sym_1 = FieldAssignment::mul(&mut ncs, &x, &x);
        let y = FieldAssignment::mul(&mut ncs, &sym_1, &x);
        let sym_2 = FieldAssignment::add(&mut ncs, &y, &x);
        FieldAssignment::eq(&mut ncs, &z, &(sym_2 + c));

        assert!(!ncs.is_sat());
    }
}
