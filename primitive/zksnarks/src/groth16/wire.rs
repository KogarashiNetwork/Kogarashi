use core::cmp::Ordering;
use core::fmt;
use core::fmt::Formatter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Wire {
    Instance(usize),
    Witness(usize),
}

impl Wire {
    pub const ONE: Wire = Wire::Instance(0);

    pub const fn deref_i(&self) -> &usize {
        match self {
            Wire::Instance(i) => i,
            Wire::Witness(i) => i,
        }
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
            self.deref_i().cmp(&other.deref_i())
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
        match self {
            Self::Instance(i) => write!(f, "instance {:?}", i),
            Self::Witness(i) => write!(f, "witness {:?}", i),
        }
    }
}
