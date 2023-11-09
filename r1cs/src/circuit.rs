use zkstd::common::PrimeField;

pub trait CircuitDriver: Clone {
    // curve base field
    type Base: PrimeField;

    // curve scalar field
    type Scalar: PrimeField;

    // bn curve 3b param
    fn b3() -> Self::Base;
}
