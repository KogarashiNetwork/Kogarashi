use super::{algebra::Field, basic::Basic, comp::ParityCmp};

/// extension field
pub trait ExtensionField: Field + Basic + ParityCmp + PartialOrd + Ord {}
