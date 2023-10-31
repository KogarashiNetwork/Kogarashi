use super::wire::{Index, Wire};
use alloc::format;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ops::{Add, Mul};
use hashbrown::HashMap;
use zkstd::common::Field;

pub trait Evaluable<F: Field, R> {
    fn evaluate(&self, instance: &Vec<(Wire, F)>, witness: &Vec<(Wire, F)>) -> R;
}

/// A linear combination of wires.
#[derive(Clone, Eq, PartialEq)]
pub struct Expression<F: Field> {
    /// The coefficient of each wire. Wires with a coefficient of zero are omitted.
    coefficients: HashMap<Wire, F>,
}

impl<F: Field> Debug for Expression<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                self.coefficients
                    .keys()
                    .map(|w| match w.get_unchecked() {
                        Index::Input(i) => format!("{i}_i"),
                        Index::Aux(i) => format!("{i}_a"),
                    })
                    .zip(self.coefficients.values()),
            )
            .finish()
    }
}

impl<F: Field> Expression<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: Vec<(Wire, F)>) -> Self {
        let nonzero_coefficients = coefficients
            .into_iter()
            .filter(|(_k, v)| *v != F::zero())
            .collect();
        Expression {
            coefficients: nonzero_coefficients,
        }
    }

    pub fn coefficients(&self) -> &HashMap<Wire, F> {
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
            self.coefficients.get(&Wire::ONE).cloned()
        } else {
            None
        }
    }

    pub fn evaluate(&self, instance: &Vec<(Wire, F)>, witness: &Vec<(Wire, F)>) -> F {
        self.coefficients
            .iter()
            .fold(F::zero(), |sum, (wire, coefficient)| {
                let wire_value = match wire.get_unchecked() {
                    Index::Input(_) => get_value_from_wire(wire.get_unchecked(), instance),
                    Index::Aux(_) => get_value_from_wire(wire.get_unchecked(), witness),
                }
                .expect("No value for the wire was found");
                sum + (wire_value * *coefficient)
            })
    }
}

fn get_value_from_wire<F: Field>(index: Index, vectors: &Vec<(Wire, F)>) -> Option<F> {
    for vector in vectors {
        if index == vector.0.get_unchecked() {
            return Some(vector.1);
        }
    }
    None
}

impl<F: Field> From<Wire> for Expression<F> {
    fn from(wire: Wire) -> Self {
        Expression::new([(wire, F::one())].iter().cloned().collect())
    }
}

impl<F: Field> From<&Wire> for Expression<F> {
    fn from(wire: &Wire) -> Self {
        Expression::from(*wire)
    }
}

impl<F: Field> From<F> for Expression<F> {
    fn from(value: F) -> Self {
        Expression::new([(Wire::ONE, value)].iter().cloned().collect())
    }
}

impl<F: Field> Add<Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: Expression<F>) -> Expression<F> {
        &self + &rhs
    }
}

impl<F: Field> Add<&Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: &Expression<F>) -> Expression<F> {
        &self + rhs
    }
}

impl<F: Field> Add<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: Expression<F>) -> Expression<F> {
        self + &rhs
    }
}

impl<F: Field> Add<&Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn add(self, rhs: &Expression<F>) -> Expression<F> {
        let mut res = Vec::new();
        for (wire, coefficient) in rhs.coefficients.clone() {
            match self.coefficients.get(&wire) {
                Some(coeff) => res.push((wire, *coeff + coefficient)),
                None => res.push((wire, coefficient)),
            }
        }
        for (wire, coefficient) in self.coefficients.clone() {
            match rhs.coefficients.get(&wire) {
                Some(_) => {}
                None => res.push((wire, coefficient)),
            }
        }
        Expression::new(res)
    }
}

#[allow(clippy::op_ref)]
impl<F: Field> Mul<F> for Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: F) -> Expression<F> {
        &self * &rhs
    }
}

impl<F: Field> Mul<&F> for Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: &F) -> Expression<F> {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl<F: Field> Mul<F> for &Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: F) -> Expression<F> {
        self * &rhs
    }
}

impl<F: Field> Mul<&F> for &Expression<F> {
    type Output = Expression<F>;

    fn mul(self, rhs: &F) -> Expression<F> {
        Expression::new(
            self.coefficients
                .iter()
                .map(|(k, v)| (*k, *v * *rhs))
                .collect(),
        )
    }
}
