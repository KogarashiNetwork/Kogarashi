#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident) => {
        group_operation!($field, $p, $g, $e);

        ring_operation!($field, $p);

        // basic trait
        impl Default for $field {
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Display for $field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use field_operation;
