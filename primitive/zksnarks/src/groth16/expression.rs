use super::matrix::Element;
use super::wire::{Index, Wire};

use core::fmt::Debug;
use core::ops::{Add, Mul};
use zkstd::common::{PrimeField, Vec};

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &Vec<Element<F>>, witness: &Vec<Element<F>>) -> R;
}

/// A linear combination of wires.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression<F: PrimeField> {
    /// The coefficient of each wire. Wires with a coefficient of zero are omitted.
    coefficients: Vec<Element<F>>,
}

impl<F: PrimeField> Expression<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: Vec<Element<F>>) -> Self {
        let nonzero_coefficients = coefficients
            .into_iter()
            .filter(|element| element.1 != F::zero())
            .collect();
        Expression {
            coefficients: nonzero_coefficients,
        }
    }

    pub fn coefficients(&self) -> &Vec<Element<F>> {
        &self.coefficients
    }

    pub fn one() -> Self {
        Expression::from(F::one())
    }

    pub fn num_terms(&self) -> usize {
        self.coefficients.len()
    }

    /// Return Some(c) if this is a constant c, otherwise None.
    pub fn as_constant(&self) -> Option<F> {
        if self.num_terms() == 1 {
            get_value_from_wire(Wire::ONE.get_unchecked(), &self.coefficients)
        } else {
            None
        }
    }

    pub fn evaluate(&self, instance: &Vec<Element<F>>, witness: &Vec<Element<F>>) -> F {
        self.coefficients
            .iter()
            .fold(F::zero(), |sum, Element(wire, coefficient)| {
                let wire_value = match wire.get_unchecked() {
                    Index::Input(_) => get_value_from_wire(wire.get_unchecked(), instance),
                    Index::Aux(_) => get_value_from_wire(wire.get_unchecked(), witness),
                }
                .expect("No value for the wire was found");
                sum + (wire_value * *coefficient)
            })
    }
}

impl<F: PrimeField> From<Wire> for Expression<F> {
    fn from(wire: Wire) -> Self {
        Expression::new([Element(wire, F::one())].iter().cloned().collect())
    }
}

impl<F: PrimeField> From<&Wire> for Expression<F> {
    fn from(wire: &Wire) -> Self {
        Expression::from(*wire)
    }
}

impl<F: PrimeField> From<F> for Expression<F> {
    fn from(value: F) -> Self {
        Expression::new([Element(Wire::ONE, value)].iter().cloned().collect())
    }
}

impl<F: PrimeField> Add<Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: Expression<F>) -> Expression<F> {
        &self + &rhs
    }
}

impl<F: PrimeField> Add<&Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: &Expression<F>) -> Expression<F> {
        &self + rhs
    }
}

impl<F: PrimeField> Add<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: Expression<F>) -> Expression<F> {
        self + &rhs
    }
}

impl<F: PrimeField> Add<&Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: &Expression<F>) -> Expression<F> {
        let mut res = Vec::new();
        let pallas = rhs.coefficients.clone();
        let velles = self.coefficients.clone();
        for Element(wire, coefficient) in pallas.clone() {
            match get_value_from_wire(wire.get_unchecked(), &velles) {
                Some(coeff) => res.push(Element(wire, coeff + coefficient)),
                None => res.push(Element(wire, coefficient)),
            }
        }
        for Element(wire, coefficient) in velles {
            match get_value_from_wire(wire.get_unchecked(), &pallas) {
                Some(_) => {}
                None => res.push(Element(wire, coefficient)),
            }
        }
        Expression::new(res)
    }
}

fn get_value_from_wire<F: PrimeField>(index: Index, vectors: &Vec<Element<F>>) -> Option<F> {
    for vector in vectors {
        if index == vector.0.get_unchecked() {
            return Some(vector.1);
        }
    }
    None
}

#[allow(clippy::op_ref)]
impl<F: PrimeField> Mul<F> for Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: F) -> Expression<F> {
        &self * &rhs
    }
}

impl<F: PrimeField> Mul<&F> for Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: &F) -> Expression<F> {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl<F: PrimeField> Mul<F> for &Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: F) -> Expression<F> {
        self * &rhs
    }
}

impl<F: PrimeField> Mul<&F> for &Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: &F) -> Expression<F> {
        Expression::new(
            self.coefficients
                .iter()
                .map(|Element(k, v)| Element(*k, *v * *rhs))
                .collect(),
        )
    }
}
