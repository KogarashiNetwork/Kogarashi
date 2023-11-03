use core::cmp::Ordering;
use core::fmt::{self, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Wire {
    Instance(usize),
    Witness(usize),
}

impl Wire {
    pub const ONE: Wire = Wire::Instance(0);
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        let rhs = match self {
            Wire::Instance(i) => i,
            Wire::Witness(i) => i,
        };
        let lhs = match other {
            Wire::Instance(i) => i,
            Wire::Witness(i) => i,
        };
        rhs.cmp(lhs)
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
