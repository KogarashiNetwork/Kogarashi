#[macro_export]
macro_rules! field_built_in {
    ($element:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $element {}

        impl Basic for $element {}

        impl Default for $element {
            fn default() -> Self {
                Self(zero())
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
macro_rules! const_field_operation {
    ($field:ident, $r:ident) => {
        impl $field {
            pub const fn zero() -> Self {
                Self(zero())
            }

            pub const fn one() -> Self {
                Self($r)
            }
        }
    };
}

pub use {const_field_operation, field_built_in};
