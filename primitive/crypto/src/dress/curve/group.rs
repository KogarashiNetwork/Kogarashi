#[macro_export]
macro_rules! projective_group_operation {
    ($projective:ident, $g:ident, $e:ident) => {
        impl Group for $projective {
            const GENERATOR: Self = $g;

            const IDENTITY: Self = $e;

            fn invert(self) -> Option<Self> {
                match self.z.is_zero() {
                    true => None,
                    false => Some(
                        Self {
                            x: self.x,
                            y: -self.y,
                            z: self.z,
                })
                }
            }
        }

        impl PartialEq for $projective {
            fn eq(&self, other: &Self) -> bool {
                (self.is_identity() && other.is_identity())
                ||
                (self.x == other.x && self.y == other.y && self.z == other.z)
            }
        }

        impl Eq for $projective {}
    };
}

pub use projective_group_operation;
