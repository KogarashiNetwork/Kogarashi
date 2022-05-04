/// Field element
pub trait Field: Eq + Copy + Clone {
    fn zero() -> Self;

    fn one() -> Self;

    fn is_zero(&self) -> bool;

    fn square(&mut self);

    fn double(&mut self);

    fn neg(&mut self);

    fn add_assign(&mut self, other: &Self);

    fn sub_assign(&mut self, other: &Self);

    fn mul_assign(&mut self, other: &Self);

    fn pow(&self, exp: u64) -> Self;
}

/// Prime filed element
pub trait PrimeField {
    const BITS: u32;

    const S: u32;

    fn root_of_unity() -> Self;
}
