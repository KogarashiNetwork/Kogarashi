#[macro_export]
macro_rules! curve_built_in {
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
                write!(f, "x: 0x")?;
                write!(f, "{:016x}\n", self.x)?;
                write!(f, "y: 0x")?;
                write!(f, "{:016x}\n", self.y)?;
                write!(f, "z: 0x")?;
                write!(f, "{:016x}\n", self.z)?;
                Ok(())
            }
        }
    };
}

pub use curve_built_in;
