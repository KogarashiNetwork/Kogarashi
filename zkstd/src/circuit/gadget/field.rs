use super::binary::BinaryAssignment;
use crate::circuit::CircuitDriver;
use crate::common::{vec, Add, Group, IntGroup, Neg, PrimeField, Ring, Sub, Vec};
use crate::matrix::SparseRow;
use crate::r1cs::{R1cs, Wire};

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

    pub fn square(cs: &mut R1cs<C>, x: &Self) -> Self {
        Self::mul(cs, x, x)
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

    pub fn range_check(cs: &mut R1cs<C>, a_bits: &[BinaryAssignment<C>], c: C::Scalar) {
        let c_bits = c
            .to_bits()
            .into_iter()
            .skip_while(|&b| b == 0)
            .collect::<Vec<_>>();

        // Check that there are no zeroes before the first one in the C
        assert!(a_bits
            .iter()
            .take(a_bits.len() - c_bits.len())
            .all(|b| cs[*b.inner()] == C::Scalar::zero()));

        let a_bits = a_bits
            .iter()
            .skip(a_bits.len() - c_bits.len())
            .collect::<Vec<_>>();

        let mut p = vec![FieldAssignment::from(a_bits[0])];
        let t = c_bits
            .iter()
            .rposition(|&b| b != 1)
            .unwrap_or(c_bits.len() - 1);

        for (&a, &c) in a_bits.iter().skip(1).zip(c_bits.iter().skip(1).take(t + 1)) {
            if c == 1 {
                p.push(FieldAssignment::mul(
                    cs,
                    p.last().unwrap(),
                    &FieldAssignment::from(a),
                ));
            } else {
                p.push(p.last().unwrap().clone());
            }
        }

        for (i, (&a, &c)) in a_bits.iter().zip(c_bits.iter()).enumerate() {
            let bit_field = FieldAssignment::from(a);
            if c == 1 {
                let bool_constr = FieldAssignment::mul(
                    cs,
                    &(&bit_field - &FieldAssignment::constant(&C::Scalar::one())),
                    &bit_field,
                );
                FieldAssignment::enforce_eq(
                    cs,
                    &bool_constr,
                    &FieldAssignment::constant(&C::Scalar::zero()),
                );
            } else if c == 0 {
                let bool_constr = FieldAssignment::mul(
                    cs,
                    &(&(&FieldAssignment::constant(&C::Scalar::one()) - &bit_field) - &p[i - 1]),
                    &bit_field,
                );
                FieldAssignment::enforce_eq(
                    cs,
                    &bool_constr,
                    &FieldAssignment::constant(&C::Scalar::zero()),
                );
            }
        }
    }

    /// To bit representation in Big-endian
    pub fn to_bits(cs: &mut R1cs<C>, x: &Self) -> Vec<BinaryAssignment<C>> {
        let bound = C::Scalar::MODULUS - C::Scalar::one();

        let bit_repr: Vec<BinaryAssignment<C>> = x
            .inner()
            .evaluate(&cs.x, &cs.w)
            .to_bits()
            .iter()
            .map(|b| BinaryAssignment::witness(cs, *b))
            .collect();
        FieldAssignment::range_check(cs, &bit_repr, bound);
        bit_repr
    }

    pub fn enforce_eq(cs: &mut R1cs<C>, x: &Self, y: &Self) {
        cs.mul_gate(&x.0, &SparseRow::one(), &y.0)
    }

    pub fn is_eq(cs: &mut R1cs<C>, x: &Self, y: &Self) -> BinaryAssignment<C> {
        let is_neq = Self::is_neq(cs, x, y);
        BinaryAssignment::not(cs, &is_neq)
    }

    pub fn is_neq(cs: &mut R1cs<C>, x: &Self, y: &Self) -> BinaryAssignment<C> {
        let x_val = x.inner().evaluate(&cs.x, &cs.w);
        let y_val = y.inner().evaluate(&cs.x, &cs.w);
        let is_not_equal = BinaryAssignment::witness(cs, if x_val != y_val { 1 } else { 0 });
        let multiplier = if x_val != y_val {
            FieldAssignment::witness(cs, (x_val - y_val).invert().unwrap())
        } else {
            FieldAssignment::constant(&C::Scalar::one())
        };

        let diff = x - y;
        let mul = FieldAssignment::mul(cs, &diff, &multiplier);
        FieldAssignment::enforce_eq(cs, &mul, &FieldAssignment::from(&is_not_equal));

        let not_is_not_equal = BinaryAssignment::not(cs, &is_not_equal);
        let mul = FieldAssignment::mul(cs, &diff, &FieldAssignment::from(&not_is_not_equal));
        FieldAssignment::enforce_eq(cs, &mul, &FieldAssignment::constant(&C::Scalar::zero()));

        is_not_equal
    }

    pub fn enforce_eq_constant(cs: &mut R1cs<C>, x: &Self, c: &C::Scalar) {
        cs.mul_gate(
            &x.0,
            &SparseRow::one(),
            &FieldAssignment::<C>::constant(c).0,
        )
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
