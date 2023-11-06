use crate::wire::Wire;

use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub struct SparseMatrix<F: PrimeField>(pub(crate) Vec<SparseRow<F>>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseRow<F: PrimeField>(pub(crate) Vec<(Wire, F)>);

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &[(Wire, F)], witness: &[(Wire, F)]) -> R;
}

impl<F: PrimeField> SparseRow<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: Vec<(Wire, F)>) -> Self {
        Self(
            coefficients
                .into_iter()
                .filter(|element| element.1 != F::zero())
                .map(|element| (element.0, element.1))
                .collect(),
        )
    }

    pub fn coefficients(&self) -> &Vec<(Wire, F)> {
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

    pub fn evaluate(&self, instance: &[(Wire, F)], witness: &[(Wire, F)]) -> F {
        self.0.iter().fold(F::zero(), |sum, (wire, coefficient)| {
            let wire_value: F = match wire {
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
        SparseRow::new([(wire, F::one())].to_vec())
    }
}

impl<F: PrimeField> From<&Wire> for SparseRow<F> {
    fn from(wire: &Wire) -> Self {
        SparseRow::from(*wire)
    }
}

impl<F: PrimeField> From<F> for SparseRow<F> {
    fn from(value: F) -> Self {
        SparseRow::new([(Wire::ONE, value)].to_vec())
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
        for (wire, coeff_b) in rhs.0.clone() {
            match get_value_from_wire::<F>(wire, &self.0) {
                Some(coeff_a) => res.push((wire, coeff_a + coeff_b)),
                None => res.push((wire, coeff_b)),
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

fn get_value_from_wire<F: PrimeField>(index: Wire, vectors: &[(Wire, F)]) -> Option<F> {
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

impl<F: PrimeField> Mul<F> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: F) -> SparseRow<F> {
        self * &rhs
    }
}

impl<F: PrimeField> Mul<&F> for &SparseRow<F> {
    type Output = SparseRow<F>;

    fn mul(self, rhs: &F) -> SparseRow<F> {
        SparseRow::new(self.0.iter().map(|(k, v)| (*k, *v * *rhs)).collect())
    }
}
