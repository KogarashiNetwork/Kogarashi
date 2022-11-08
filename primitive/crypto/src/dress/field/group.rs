#[macro_export]
macro_rules! group_operation {
    ($field:ident, $g:ident, $e:ident) => {
        impl Group for $field {
            const GENERATOR: Self = $g;

            const IDENTITY: Self = $e;

            fn invert(self) -> Self {
                Self(invert(&self.0, &$p.0).unwrap())
            }
        }

        impl PartialEq for $field {
            fn eq(&self, other: &Self) -> bool {
                self.0[0] == other.0[0]
                    && self.0[1] == other.0[1]
                    && self.0[2] == other.0[2]
                    && self.0[3] == other.0[3]
            }
        }

        // comparison operation
        impl Eq for Fr {}

        impl PartialOrd for $field {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }

            fn lt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                false
            }

            fn le(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                true
            }

            fn gt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                false
            }

            fn ge(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                true
            }
        }

        impl Ord for Fr {
            fn cmp(&self, other: &Self) -> Ordering {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a < b {
                        return Ordering::Less;
                    } else if a > b {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
        }
    };
}

pub use group_operation;
