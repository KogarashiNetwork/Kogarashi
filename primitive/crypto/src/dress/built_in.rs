#[macro_export]
macro_rules! field_built_in {
    ($element:ident) => {
        impl ParityCmp for $element {}
        impl Basic for $element {}

        impl Display for $element {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }

        impl LowerHex for $element {
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
