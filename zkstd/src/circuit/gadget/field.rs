use super::binary::BinaryAssignment;
use crate::circuit::CircuitDriver;
use crate::common::{vec, Add, Neg, PrimeField, Sub, Vec};
use crate::matrix::SparseRow;
use crate::r1cs::{R1cs, Wire};

#[derive(Clone)]
pub struct FieldAssignment<F: PrimeField>(SparseRow<F>);

impl<F: PrimeField> FieldAssignment<F> {
    pub fn inner(&self) -> &SparseRow<F> {
        &self.0
    }
    pub fn instance<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, instance: F) -> Self {
        let wire = cs.public_wire();
        cs.x.push(instance);

        Self(SparseRow::from(wire))
    }

    pub fn witness<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, witness: F) -> Self {
        let wire = cs.private_wire();
        cs.w.push(witness);

        Self(SparseRow::from(wire))
    }

    pub fn constant(constant: &F) -> Self {
        Self(SparseRow(vec![(Wire::ONE, *constant)]))
    }

    pub fn square<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, x: &Self) -> Self {
        Self::mul(cs, x, x)
    }

    pub fn mul<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, x: &Self, y: &Self) -> Self {
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

    pub fn add<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, x: &Self, y: &Self) -> Self {
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

    pub fn range_check<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        a_bits: &[BinaryAssignment],
        c: F,
    ) {
        let c_bits = c
            .to_bits()
            .into_iter()
            .skip_while(|&b| b == 0)
            .collect::<Vec<_>>();

        // Check that there are no zeroes before the first one in the C
        assert!(a_bits
            .iter()
            .take(a_bits.len() - c_bits.len())
            .all(|b| cs[*b.inner()] == F::zero()));

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
                    &(&bit_field - &FieldAssignment::constant(&F::one())),
                    &bit_field,
                );
                FieldAssignment::enforce_eq(
                    cs,
                    &bool_constr,
                    &FieldAssignment::constant(&F::zero()),
                );
            } else if c == 0 {
                let bool_constr = FieldAssignment::mul(
                    cs,
                    &(&(&FieldAssignment::constant(&F::one()) - &bit_field) - &p[i - 1]),
                    &bit_field,
                );
                FieldAssignment::enforce_eq(
                    cs,
                    &bool_constr,
                    &FieldAssignment::constant(&F::zero()),
                );
            }
        }
    }

    /// To bit representation in Big-endian
    pub fn to_bits<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        x: &Self,
    ) -> Vec<BinaryAssignment> {
        let bound = F::MODULUS - F::one();

        let bit_repr: Vec<BinaryAssignment> = x
            .inner()
            .evaluate(&cs.x, &cs.w)
            .to_bits()
            .iter()
            .map(|b| BinaryAssignment::witness(cs, *b))
            .collect();
        FieldAssignment::range_check(cs, &bit_repr, bound);
        bit_repr
    }

    pub fn enforce_eq<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, x: &Self, y: &Self) {
        cs.mul_gate(&x.0, &SparseRow::one(), &y.0)
    }

    pub fn conditional_enforce_equal<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        x: &Self,
        y: &Self,
        should_enforce: &BinaryAssignment,
    ) {
        let mul = FieldAssignment::mul(cs, &(x - y), &FieldAssignment::from(should_enforce));
        FieldAssignment::enforce_eq_constant(cs, &mul, &F::zero());
    }

    pub fn is_eq<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        x: &Self,
        y: &Self,
    ) -> BinaryAssignment {
        let is_neq = Self::is_neq(cs, x, y);
        BinaryAssignment::not(cs, &is_neq)
    }

    pub fn is_neq<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        x: &Self,
        y: &Self,
    ) -> BinaryAssignment {
        let x_val = x.inner().evaluate(&cs.x, &cs.w);
        let y_val = y.inner().evaluate(&cs.x, &cs.w);
        let is_not_equal = BinaryAssignment::witness(cs, u8::from(x_val != y_val));
        let multiplier = if x_val != y_val {
            FieldAssignment::witness(cs, (x_val - y_val).invert().unwrap())
        } else {
            FieldAssignment::witness(cs, F::one())
        };

        let diff = x - y;
        let mul = FieldAssignment::mul(cs, &diff, &multiplier);
        FieldAssignment::enforce_eq(cs, &mul, &FieldAssignment::from(&is_not_equal));

        let not_is_not_equal = BinaryAssignment::not(cs, &is_not_equal);
        let mul = FieldAssignment::mul(cs, &diff, &FieldAssignment::from(&not_is_not_equal));
        FieldAssignment::enforce_eq(cs, &mul, &FieldAssignment::constant(&F::zero()));

        is_not_equal
    }

    pub fn enforce_eq_constant<C: CircuitDriver<Scalar = F>>(cs: &mut R1cs<C>, x: &Self, c: &F) {
        cs.mul_gate(&x.0, &SparseRow::one(), &FieldAssignment::constant(c).0)
    }
}

impl<F: PrimeField> From<&BinaryAssignment> for FieldAssignment<F> {
    fn from(value: &BinaryAssignment) -> Self {
        Self(SparseRow::from(value.inner()))
    }
}

impl<F: PrimeField> Add<&FieldAssignment<F>> for &FieldAssignment<F> {
    type Output = FieldAssignment<F>;

    fn add(self, rhs: &FieldAssignment<F>) -> Self::Output {
        FieldAssignment(&self.0 + &rhs.0)
    }
}

impl<F: PrimeField> Sub<&FieldAssignment<F>> for &FieldAssignment<F> {
    type Output = FieldAssignment<F>;

    fn sub(self, rhs: &FieldAssignment<F>) -> Self::Output {
        FieldAssignment(&self.0 - &rhs.0)
    }
}

impl<F: PrimeField> Neg for &FieldAssignment<F> {
    type Output = FieldAssignment<F>;

    fn neg(self) -> Self::Output {
        FieldAssignment(-&self.0)
    }
}
