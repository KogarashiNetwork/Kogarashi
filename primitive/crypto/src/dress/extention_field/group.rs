#[macro_export]
macro_rules! extention_field_group_operation {
    ($extention_field:ident, $g:ident) => {
        impl Group for $extention_field {
            const GENERATOR: Self = $extention_field::zero();

            const IDENTITY: Self = $extention_field::zero();

            fn invert(self) -> Option<Self> {
                Some(self)
            }
        }

        impl PartialEq for $extention_field {
            fn eq(&self, other: &Self) -> bool {
                self.0[0].0[0] == other.0[0].0[0]
                    && self.0[0].0[1] == other.0[0].0[1]
                    && self.0[0].0[2] == other.0[0].0[2]
                    && self.0[0].0[3] == other.0[0].0[3]
                    && self.0[1].0[0] == other.0[1].0[0]
                    && self.0[1].0[1] == other.0[1].0[1]
                    && self.0[1].0[2] == other.0[1].0[2]
                    && self.0[1].0[3] == other.0[1].0[3]
            }
        }

        impl Eq for $extention_field {}
    };
}

pub use extention_field_group_operation;
