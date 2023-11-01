use core::cmp::Ordering;
use core::fmt;
use core::fmt::Formatter;
use core::ops::Deref;

/// Represents the index of either an input variable or
/// auxiliary variable.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum Index {
    Input(usize),
    Aux(usize),
}

impl Deref for Index {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            Index::Input(i) => i,
            Index::Aux(i) => i,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Wire(pub(crate) Index);

impl Wire {
    pub const ONE: Wire = Wire(Index::Input(0));

    pub const fn get_unchecked(self) -> Index {
        self.0
    }
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        // For presentation, we want the 1 wire to be last. Otherwise use ascending index order.
        if *self == Wire::ONE && *other == Wire::ONE {
            Ordering::Equal
        } else if *self == Wire::ONE {
            Ordering::Less
        } else if *other == Wire::ONE {
            Ordering::Greater
        } else {
            self.get_unchecked().cmp(&other.get_unchecked())
        }
    }
}

impl PartialOrd for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Index::Input(0) = self.0 {
            write!(f, "1")
        } else {
            write!(f, "wire_{:?}", self.0)
        }
    }
}
