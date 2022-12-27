#[macro_export]
macro_rules! extension_field_group_operation {
    ($extension_field:ident) => {
        impl Group for $extension_field {
            const GENERATOR: Self = $extension_field::dummy();

            const IDENTITY: Self = $extension_field::dummy();

            fn invert(self) -> Option<Self> {
                self.get_invert()
            }
        }

        impl PartialEq for $extension_field {
            fn eq(&self, other: &Self) -> bool {
                let mut acc = true;
                self.0.iter().zip(other.0.iter()).for_each(|(a, b)| {
                    acc = acc && a == b;
                });
                acc
            }
        }

        impl Eq for $extension_field {}
    };
}

pub use extension_field_group_operation;
