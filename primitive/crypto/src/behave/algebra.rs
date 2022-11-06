use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Group: PartialEq + PartialOrd + Ord {
    const GENERATOR: Self;

    const IDENTITY: Self;

    fn invert(self) -> Self;
}

pub trait Ring: Group + Add + AddAssign + Mul + MulAssign + Neg + Sub + SubAssign + Sized {}

pub trait Field: Ring + Div + DivAssign {}
