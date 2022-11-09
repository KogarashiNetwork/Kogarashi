#[macro_export]
macro_rules! group_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident) => {
        impl Group for $field {
            const GENERATOR: Self = $field($g);

            const IDENTITY: Self = $field($e);

            fn invert(self) -> Option<Self> {
                match invert(self.0, $p, $inv) {
                    Some(x) => Some(Self(x)),
                    None => None,
                }
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

        impl Eq for $field {}
    };
}

pub use group_operation;
