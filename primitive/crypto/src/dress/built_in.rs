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

#[macro_export]
macro_rules! curve_built_in {
    ($affine:ident, $projective:ident) => {
        impl ParityCmp for $affine {}
        impl ParityCmp for $projective {}
        impl Basic for $affine {}
        impl Basic for $projective {}

        impl Default for $affine {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl Default for $projective {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl From<$affine> for $projective {
            fn from(a: $affine) -> $projective {
                a.to_projective()
            }
        }

        impl From<$projective> for $affine {
            fn from(p: $projective) -> $affine {
                p.to_affine()
            }
        }

        impl Display for $affine {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "x: 0x")?;
                for i in self.x.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " y: 0x")?;
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
                    write!(f, "{:?}", *i)?;
                }
                write!(f, " y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:?}", *i)?;
                }
                write!(f, " z: 0x")?;
                for i in self.z.0.iter().rev() {
                    write!(f, "{:?}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use {curve_built_in, field_built_in};
