use core::cmp::Ordering;
use core::fmt;
use core::fmt::Formatter;

/// A wire represents a witness element.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Wire {
    pub index: u32,
}

impl Wire {
    pub const ONE: Wire = Wire { index: 0 };
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self == Wire::ONE && *other == Wire::ONE {
            Ordering::Equal
        } else if *self == Wire::ONE {
            Ordering::Greater
        } else if *other == Wire::ONE {
            Ordering::Less
        } else {
            self.index.cmp(&other.index)
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
        if self.index == 0 {
            write!(f, "1")
        } else {
            write!(f, "wire_{}", self.index)
        }
    }
}
