mod element;
use super::wire::Wire;

pub(crate) use element::Element;

use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<SparseRow<F>>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseRow<F: PrimeField>(pub(crate) Vec<Element<F>>);

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> R;
}

impl<F: PrimeField> SparseRow<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: Vec<Element<F>>) -> Self {
        Self(
            coefficients
                .into_iter()
                .filter(|element| element.1 != F::zero())
                .collect(),
        )
    }

    pub fn coefficients(&self) -> &Vec<Element<F>> {
        &self.0
    }

    pub fn one() -> Self {
        Self::from(F::one())
    }

    pub fn num_terms(&self) -> usize {
        self.0.len()
    }

    /// Return Some(c) if this is a constant c, otherwise None.
    pub fn as_constant(&self) -> Option<F> {
        if self.num_terms() == 1 {
            get_value_from_wire(Wire::ONE, &self.0)
        } else {
            None
        }
    }

    pub fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> F {
        self.0
            .iter()
            .fold(F::zero(), |sum, Element(wire, coefficient)| {
                let wire_value = match wire {
                    Wire::Instance(_) => get_value_from_wire(*wire, instance),
                    Wire::Witness(_) => get_value_from_wire(*wire, witness),
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
        let mut res = self.0.clone();
        for Element(wire, coeff_b) in rhs.0.clone() {
            match get_value_from_wire(wire, &self.0) {
                Some(coeff_a) => res.push(Element(wire, coeff_a + coeff_b)),
                None => res.push(Element(wire, coeff_b)),
            }
        }
        SparseRow::new(res)
    }
}

impl<F: PrimeField> Sub<SparseRow<F>> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn sub(self, rhs: SparseRow<F>) -> Self::Output {
        &self - &rhs
    }
}

impl<F: PrimeField> Sub<&SparseRow<F>> for SparseRow<F> {
    type Output = SparseRow<F>;

    fn sub(self, rhs: &SparseRow<F>) -> Self::Output {
        &self - rhs
    }
}

impl<F: PrimeField> Sub<SparseRow<F>> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn sub(self, rhs: SparseRow<F>) -> Self::Output {
        self - &rhs
    }
}

impl<F: PrimeField> Sub<&SparseRow<F>> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn sub(self, rhs: &SparseRow<F>) -> Self::Output {
        self + -rhs
    }
}

impl<F: PrimeField> Neg for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn neg(self) -> SparseRow<F> {
        self * -F::one()
    }
}

impl<F: PrimeField> Neg for SparseRow<F> {
    type Output = SparseRow<F>;

    fn neg(self) -> SparseRow<F> {
        -&self
    }
}

fn get_value_from_wire<F: PrimeField>(index: Wire, vectors: &[Element<F>]) -> Option<F> {
    for vector in vectors {
        if index == vector.0 {
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
            self.0
                .iter()
                .map(|Element(k, v)| Element(*k, *v * *rhs))
                .collect(),
        )
    }
}
