#[macro_export]
macro_rules! field_built_in {
    ($element:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $element {}

        impl Basic for $element {}

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

pub use field_built_in;
