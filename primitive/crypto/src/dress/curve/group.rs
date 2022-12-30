#[macro_export]
macro_rules! affine_group_operation {
    ($affine:ident, $g:ident, $e:ident) => {
        impl Group for $affine {
            const GENERATOR: Self = Self {
                x: $g.x,
                y: $g.y,
                is_infinity: false,
            };

            const IDENTITY: Self = Self {
                x: $e.x,
                y: $e.y,
                is_infinity: false,
            };

            fn invert(self) -> Option<Self> {
                match self.is_infinity {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                        is_infinity: false,
                    }),
                }
            }
        }

        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x == other.x && self.y == other.y
                }
            }
        }

        impl Eq for $affine {}
    };
}

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
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z
                }
            }
        }

        impl Eq for $projective {}
    };
}

pub use {affine_group_operation, projective_group_operation};
