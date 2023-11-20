use crate::driver::CircuitDriver;
use crate::matrix::SparseRow;
use crate::wire::Wire;
use crate::R1cs;
use std::ops::{Neg, Sub};

use crate::gadget::binary::BinaryAssignment;
use zkstd::common::{Add, PrimeField};

#[derive(Clone)]
pub struct FieldAssignment<C: CircuitDriver>(SparseRow<C::Scalar>);

impl<C: CircuitDriver> FieldAssignment<C> {
    pub fn inner(&self) -> &SparseRow<C::Scalar> {
        &self.0
    }
    pub fn instance(cs: &mut R1cs<C>, instance: C::Scalar) -> Self {
        let wire = cs.public_wire();
        cs.x.push(instance);

        Self(SparseRow::from(wire))
    }

    pub fn witness(cs: &mut R1cs<C>, witness: C::Scalar) -> Self {
        let wire = cs.private_wire();
        cs.w.push(witness);

        Self(SparseRow::from(wire))
    }

    pub fn constant(constant: &C::Scalar) -> Self {
        Self(SparseRow(vec![(Wire::ONE, *constant)]))
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

    fn range_check(cs: &mut R1cs<C>, value: &Self, num_of_bits: u16) {
        assert!(0 < num_of_bits && num_of_bits <= C::NUM_BITS);
        fn inner_product<C: CircuitDriver>(
            cs: &mut R1cs<C>,
            a: &[u8],
            b: &[C::Scalar],
        ) -> FieldAssignment<C> {
            if a.len() == 1 && b.len() == 1 {
                let bit = BinaryAssignment::witness(cs, a[0]);
                FieldAssignment::mul(
                    cs,
                    &FieldAssignment::from(&bit),
                    &FieldAssignment::constant(&b[0]),
                )
            } else {
                &inner_product(cs, &a[0..a.len() / 2], &b[0..b.len() / 2])
                    + &inner_product(cs, &a[a.len() / 2..], &b[b.len() / 2..])
            }
        }

        let powers_of_2 = (0..num_of_bits)
            .map(|i| C::Scalar::pow_of_2(i as u64))
            .collect::<Vec<_>>();
        let mut bits = value.inner().evaluate(&cs.x, &cs.w).to_bits();
        bits.reverse();
        bits.resize(num_of_bits as usize, 0);

        let inner_product = inner_product(cs, &bits, &powers_of_2);
        FieldAssignment::eq(cs, value, &inner_product);
    }

    /// To bit representation in Big-endian
    pub fn to_bits(cs: &mut R1cs<C>, x: &Self) -> Vec<BinaryAssignment<C>> {
        FieldAssignment::range_check(cs, x, C::NUM_BITS);
        let decomposition: Vec<BinaryAssignment<C>> = x
            .inner()
            .evaluate(&cs.x, &cs.w)
            .to_bits()
            .iter()
            .map(|b| BinaryAssignment::witness(cs, *b))
            .collect();

        decomposition
    }

    pub fn eq(cs: &mut R1cs<C>, x: &Self, y: &Self) {
        cs.mul_gate(&x.0, &SparseRow::one(), &y.0)
    }
}

impl<C: CircuitDriver> From<&BinaryAssignment<C>> for FieldAssignment<C> {
    fn from(value: &BinaryAssignment<C>) -> Self {
        Self(SparseRow::from(value.inner()))
    }
}

impl<C: CircuitDriver> Add<&FieldAssignment<C>> for &FieldAssignment<C> {
    type Output = FieldAssignment<C>;

    fn add(self, rhs: &FieldAssignment<C>) -> Self::Output {
        FieldAssignment(&self.0 + &rhs.0)
    }
}

impl<C: CircuitDriver> Sub<&FieldAssignment<C>> for &FieldAssignment<C> {
    type Output = FieldAssignment<C>;

    fn sub(self, rhs: &FieldAssignment<C>) -> Self::Output {
        FieldAssignment(&self.0 - &rhs.0)
    }
}

impl<C: CircuitDriver> Neg for &FieldAssignment<C> {
    type Output = FieldAssignment<C>;

    fn neg(self) -> Self::Output {
        FieldAssignment(-&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldAssignment, R1cs};
    use crate::driver::GrumpkinDriver;
    use crate::gadget::binary::BinaryAssignment;
    use bn_254::{Fr as Scalar, Fr};
    use ff::PrimeField as FFPrimeField;
    use zkstd::common::{Group, OsRng, PrimeField};

    #[test]
    fn to_bits() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let input = Fr::random(OsRng);

        let x = FieldAssignment::instance(&mut cs, input);
        let _bits: Vec<BinaryAssignment<GrumpkinDriver>> = FieldAssignment::to_bits(&mut cs, &x);

        assert!(cs.is_sat());
    }

    #[test]
    fn field_range() {
        for i in 1..Fr::NUM_BITS {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let mut ncs = cs.clone();
            let x = Fr::pow_of_2(i as u64);

            let x_ass = FieldAssignment::instance(&mut cs, x - Fr::one());
            FieldAssignment::range_check(&mut cs, &x_ass, i as u16);
            assert!(cs.is_sat());

            let x_ass = FieldAssignment::instance(&mut ncs, x);
            FieldAssignment::range_check(&mut ncs, &x_ass, i as u16);
            assert!(!ncs.is_sat());
        }
    }

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
        let sum = &x + &y;
        FieldAssignment::eq(&mut cs, &z, &sum);

        assert!(cs.is_sat());

        // a + b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let sum = &x + &y;
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
        let sym_2 = &y + &x;
        FieldAssignment::eq(&mut cs, &z, &(&sym_2 + &c));

        assert!(cs.is_sat());

        // x^3 + x + 5 != 36
        let c = Scalar::from(5);
        let out = Scalar::from(36);
        let x = FieldAssignment::witness(&mut ncs, input);
        let c = FieldAssignment::constant(&c);
        let z = FieldAssignment::instance(&mut ncs, out);
        let sym_1 = FieldAssignment::mul(&mut ncs, &x, &x);
        let y = FieldAssignment::mul(&mut ncs, &sym_1, &x);
        let sym_2 = &y + &x;
        FieldAssignment::eq(&mut ncs, &z, &(&sym_2 + &c));

        assert!(!ncs.is_sat());
    }
}
