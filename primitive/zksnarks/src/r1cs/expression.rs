use crate::r1cs::util::join;
use crate::r1cs::wire::Wire;
use crate::r1cs::wire_values::WireValues;
#[cfg(not(feature = "std"))]
use alloc::collections::btree_map::BTreeMap;
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Formatter;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use itertools::Itertools;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeSet;
use zkstd::common::Field;

/// A linear combination of wires.
#[derive(Debug, Eq, PartialEq)]
pub struct Expression<F: Field> {
    /// The coefficient of each wire. Wires with a coefficient of zero are omitted.
    coefficients: BTreeMap<Wire, F>,
}

impl<F: Field> Expression<F> {
    /// Creates a new expression with the given wire coefficients.
    pub fn new(coefficients: BTreeMap<Wire, F>) -> Self {
        let nonzero_coefficients = coefficients
            .into_iter()
            .filter(|(_k, v)| *v != F::zero())
            .collect();
        Expression {
            coefficients: nonzero_coefficients,
        }
    }

    pub fn coefficients(&self) -> &BTreeMap<Wire, F> {
        &self.coefficients
    }

    /// The sum of zero or more wires, each with an implied coefficient of 1.
    pub fn sum_of_wires(wires: &[Wire]) -> Self {
        Expression {
            coefficients: wires.iter().map(|&v| (v, F::one())).collect(),
        }
    }

    /// The collectivization of all existing Expression’s Wires with each destination Wire’s
    /// coefficient the sum of each source’s coefficients.
    pub fn sum_of_expressions(expressions: &[Expression<F>]) -> Self {
        let mut merged_coefficients = BTreeMap::new();
        for exp in expressions {
            for (&wire, coefficient) in &exp.coefficients {
                *merged_coefficients.entry(wire).or_insert_with(F::zero) += *coefficient
            }
        }
        Expression::new(merged_coefficients)
    }

    pub fn zero() -> Self {
        Expression {
            coefficients: BTreeMap::new(),
        }
    }

    pub fn one() -> Self {
        Expression::from(F::one())
    }

    /// The additive inverse of 1.
    pub fn neg_one() -> Self {
        -Expression::one()
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

    /// Return a vector of all wires that this expression depends on.
    pub fn dependencies(&self) -> Vec<Wire> {
        self.coefficients.keys().copied().collect()
    }

    pub fn evaluate(&self, wire_values: &WireValues<F>) -> F {
        self.coefficients
            .iter()
            .fold(F::zero(), |sum, (wire, coefficient)| {
                sum + (*wire_values.get(*wire) * *coefficient)
            })
    }
}

impl<F: Field> Clone for Expression<F> {
    fn clone(&self) -> Self {
        Expression {
            coefficients: self.coefficients.clone(),
        }
    }
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

impl<F: Field> From<&F> for Expression<F> {
    fn from(value: &F) -> Self {
        Expression::from(*value)
    }
}

impl<F: Field> Neg for &Expression<F> {
    type Output = Expression<F>;

    fn neg(self) -> Expression<F> {
        self * -F::one()
    }
}

impl<F: Field> Neg for Expression<F> {
    type Output = Expression<F>;

    fn neg(self) -> Expression<F> {
        -&self
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
        // TODO: Use Expression::sum_of_expressions
        let mut merged_coefficients = self.coefficients.clone();
        for (wire, coefficient) in rhs.coefficients.clone() {
            *merged_coefficients.entry(wire).or_insert_with(F::zero) += coefficient
        }
        Expression::new(merged_coefficients)
    }
}

impl<F: Field> AddAssign for Expression<F> {
    fn add_assign(&mut self, rhs: Expression<F>) {
        *self += &rhs;
    }
}

impl<F: Field> AddAssign<&Expression<F>> for Expression<F> {
    fn add_assign(&mut self, rhs: &Expression<F>) {
        // TODO: Merge coefficients instead.
        *self = self.clone() + rhs;
    }
}

impl<F: Field> Sub<Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn sub(self, rhs: Expression<F>) -> Self::Output {
        &self - &rhs
    }
}

impl<F: Field> Sub<&Expression<F>> for Expression<F> {
    type Output = Expression<F>;

    fn sub(self, rhs: &Expression<F>) -> Self::Output {
        &self - rhs
    }
}

impl<F: Field> Sub<Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn sub(self, rhs: Expression<F>) -> Self::Output {
        self - &rhs
    }
}

impl<F: Field> Sub<&Expression<F>> for &Expression<F> {
    type Output = Expression<F>;

    fn sub(self, rhs: &Expression<F>) -> Self::Output {
        self + -rhs
    }
}

impl<F: Field> SubAssign for Expression<F> {
    fn sub_assign(&mut self, rhs: Expression<F>) {
        *self -= &rhs;
    }
}

impl<F: Field> SubAssign<&Expression<F>> for Expression<F> {
    fn sub_assign(&mut self, rhs: &Expression<F>) {
        *self = &*self - rhs;
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

impl<F: Field> MulAssign<F> for Expression<F> {
    fn mul_assign(&mut self, rhs: F) {
        *self *= &rhs;
    }
}

impl<F: Field> MulAssign<&F> for Expression<F> {
    fn mul_assign(&mut self, rhs: &F) {
        *self = self.clone() * rhs;
    }
}

#[allow(clippy::op_ref)]
impl<F: Field> Div<F> for Expression<F> {
    type Output = Expression<F>;

    fn div(self, rhs: F) -> Expression<F> {
        &self / &rhs
    }
}

impl<F: Field> Div<&F> for Expression<F> {
    type Output = Expression<F>;

    fn div(self, rhs: &F) -> Expression<F> {
        &self / rhs
    }
}

#[allow(clippy::op_ref)]
impl<F: Field> Div<F> for &Expression<F> {
    type Output = Expression<F>;

    fn div(self, rhs: F) -> Expression<F> {
        self / &rhs
    }
}

impl<F: Field> Div<&F> for &Expression<F> {
    type Output = Expression<F>;

    fn div(self, rhs: &F) -> Expression<F> {
        Expression::new(
            self.coefficients
                .iter()
                .map(|(k, v)| (*k, *v / *rhs))
                .collect(),
        )
    }
}

impl<F: Field> DivAssign<F> for Expression<F> {
    fn div_assign(&mut self, rhs: F) {
        *self /= &rhs;
    }
}

impl<F: Field> DivAssign<&F> for Expression<F> {
    fn div_assign(&mut self, rhs: &F) {
        let self_immutable: &Expression<F> = self;
        *self = self_immutable / rhs;
    }
}

impl<F: Field> fmt::Display for Expression<F> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let term_strings: Vec<String> = self
            .coefficients
            .iter()
            .sorted_by(|(k1, _v1), (k2, _v2)| k1.cmp(k2))
            .map(|(k, v)| {
                if *k == Wire::ONE {
                    format!("{:?}", v)
                } else if *v == F::one() {
                    format!("{}", k)
                } else {
                    format!("{} * {:?}", k, v)
                }
            })
            .collect();
        let s = if term_strings.is_empty() {
            String::from("0")
        } else {
            join(" + ", &term_strings)
        };
        write!(f, "{}", s)
    }
}
