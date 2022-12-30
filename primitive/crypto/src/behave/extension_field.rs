use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
};

/// extension field
pub trait ExtensionField: Field + Basic + ParityCmp + PartialOrd + Ord {
    fn mul_by_nonresidue(self) -> Self;
}
