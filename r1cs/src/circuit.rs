use zkstd::common::PrimeField;

pub trait CircuitDriver {
    // curve base field
    type Base: PrimeField;

    // curve scalar field
    type Scalar: PrimeField;

    // bn curve b param
    fn b() -> Self::Base;
}
