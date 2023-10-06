use core::cmp::Ordering;
use core::fmt;
use core::fmt::Formatter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Wire(usize);

impl Wire {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl Wire {
    /// A special wire whose value is always set to 1. This is used to create `Expression`s with
    /// constant terms.
    pub const ONE: Wire = Wire(0);
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        // For presentation, we want the 1 wire to be last. Otherwise use ascending index order.
        if *self == Wire::ONE && *other == Wire::ONE {
            Ordering::Equal
        } else if *self == Wire::ONE {
            Ordering::Greater
        } else if *other == Wire::ONE {
            Ordering::Less
        } else {
            self.0.cmp(&other.0)
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
        if self.0 == 0 {
            write!(f, "1")
        } else {
            write!(f, "wire_{}", self.0)
        }
    }
}
