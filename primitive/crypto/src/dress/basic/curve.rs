#[macro_export]
macro_rules! curve_built_in {
    ($affine:ident, $projective:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $affine {}

        impl ParityCmp for $projective {}

        impl Basic for $affine {}

        impl Basic for $projective {}

        impl Default for $affine {
            fn default() -> Self {
                unimplemented!()
            }
        }

        impl Default for $projective {
            fn default() -> Self {
                Self::GENERATOR
            }
        }

        impl Display for $affine {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "x: 0x")?;
                for i in self.x.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, "y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
        impl Display for $projective {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "x: 0x")?;
                for i in self.x.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, "y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, "z: 0x")?;
                for i in self.z.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use curve_built_in;
