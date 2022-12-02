#[macro_export]
macro_rules! extention_field_group_operation {
    ($extention_field:ident) => {
        impl Group for $extention_field {
            const GENERATOR: Self = $extention_field::dummy();

            const IDENTITY: Self = $extention_field::dummy();

            fn invert(self) -> Option<Self> {
                match self.is_zero() {
                    true => None,
                    _ => {
                        let t = self.0[0].square() + self.0[1].square();
                        let t_inv = t.invert().unwrap();
                        Some($extention_field([t_inv * self.0[0], t_inv * -self.0[1]]))
                    }
                }
            }
        }

        impl PartialEq for $extention_field {
            fn eq(&self, other: &Self) -> bool {
                let mut acc = true;
                self.0.iter().zip(other.0.iter()).for_each(|(a, b)| {
                    acc = acc && a == b;
                });
                acc
            }
        }

        impl Eq for $extention_field {}
    };
}

pub use extention_field_group_operation;
