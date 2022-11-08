#[macro_export]
macro_rules! built_in_operation {
    ($element:ident) => {
        impl ParityCmp for $element {}

        impl Basic for $element {}

        // basic trait
        impl Default for $element {
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Display for $element {
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

pub use built_in_operation;
