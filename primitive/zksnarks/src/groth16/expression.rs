use super::matrix::Element;
use super::wire::{Index, Wire};

use core::fmt::Debug;
use core::ops::{Add, Mul};
use zkstd::common::{PrimeField, Vec};

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> R;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseRow<F: PrimeField> {
    /// The coefficient of each wire. Wires with a coefficient of zero are omitted.
    elements: Vec<Element<F>>,
}

impl<F: PrimeField> SparseRow<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: Vec<Element<F>>) -> Self {
        Self {
            elements: coefficients
                .into_iter()
                .filter(|element| element.1 != F::zero())
                .collect(),
        }
    }

    pub fn coefficients(&self) -> &Vec<Element<F>> {
        &self.elements
    }

    pub fn one() -> Self {
        Self::from(F::one())
    }

    pub fn num_terms(&self) -> usize {
        self.elements.len()
    }

    /// Return Some(c) if this is a constant c, otherwise None.
    pub fn as_constant(&self) -> Option<F> {
        if self.num_terms() == 1 {
            get_value_from_wire(Wire::ONE.get_unchecked(), &self.elements)
        } else {
            None
        }
    }

    pub fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> F {
        self.elements
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

impl<F: PrimeField> From<Wire> for SparseRow<F> {
    fn from(wire: Wire) -> Self {
        SparseRow::new([Element(wire, F::one())].to_vec())
    }
}

impl<F: PrimeField> From<&Wire> for SparseRow<F> {
    fn from(wire: &Wire) -> Self {
        SparseRow::from(*wire)
    }
}

impl<F: PrimeField> From<F> for SparseRow<F> {
    fn from(value: F) -> Self {
        SparseRow::new([Element(Wire::ONE, value)].to_vec())
    }
}

impl<F: PrimeField> Add<SparseRow<F>> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn add(self, rhs: SparseRow<F>) -> SparseRow<F> {
        &self + &rhs
    }
}

impl<F: PrimeField> Add<&SparseRow<F>> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn add(self, rhs: &SparseRow<F>) -> SparseRow<F> {
        &self + rhs
    }
}

impl<F: PrimeField> Add<SparseRow<F>> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn add(self, rhs: SparseRow<F>) -> SparseRow<F> {
        self + &rhs
    }
}

impl<F: PrimeField> Add<&SparseRow<F>> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn add(self, rhs: &SparseRow<F>) -> SparseRow<F> {
        let mut res = Vec::new();
        for Element(wire, coefficient) in rhs.elements.clone() {
            match get_value_from_wire(wire.get_unchecked(), &self.elements) {
                Some(coeff) => res.push(Element(wire, coeff + coefficient)),
                None => res.push(Element(wire, coefficient)),
            }
        }
        for Element(wire, coefficient) in self.elements.clone() {
            match get_value_from_wire(wire.get_unchecked(), &rhs.elements) {
                Some(_) => {}
                None => res.push(Element(wire, coefficient)),
            }
        }
        SparseRow::new(res)
    }
}

fn get_value_from_wire<F: PrimeField>(index: Index, vectors: &[Element<F>]) -> Option<F> {
    for vector in vectors {
        if index == vector.0.get_unchecked() {
            return Some(vector.1);
        }
    }
    None
}

#[allow(clippy::op_ref)]
impl<F: PrimeField> Mul<F> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: F) -> SparseRow<F> {
        &self * &rhs
    }
}

impl<F: PrimeField> Mul<&F> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: &F) -> SparseRow<F> {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl<F: PrimeField> Mul<F> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: F) -> SparseRow<F> {
        self * &rhs
    }
}

impl<F: PrimeField> Mul<&F> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: &F) -> SparseRow<F> {
        SparseRow::new(
            self.elements
                .iter()
                .map(|Element(k, v)| Element(*k, *v * *rhs))
                .collect(),
        )
    }
}
