#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wire {
    /// wire for constant one, public input and output
    Instance(usize),
    /// wire for private input and intermediate value
    Witness(usize),
    /// wire for first element one
    One,
}

impl Wire {
    pub(crate) fn instance(index: usize) -> Self {
        Self::Instance(index)
    }

    pub(crate) fn witness(index: usize) -> Self {
        Self::Witness(index)
    }

    pub(crate) fn one() -> Self {
        Self::One
    }
}
