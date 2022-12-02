#[macro_export]
macro_rules! extention_field_built_in {
    ($extention_field:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $extention_field {}

        impl Basic for $extention_field {}

        impl Default for $extention_field {
            fn default() -> Self {
                Self::zero()
            }
        }

        impl Display for $extention_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0[0].0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " + 0x")?;
                for i in self.0[1].0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, "*u")?;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! const_extention_field_operation {
    ($extention_field:ident, $sub_field:ident) => {
        impl $extention_field {
            pub const fn zero() -> Self {
                Self([$sub_field::zero(), $sub_field::zero()])
            }

            pub const fn one() -> Self {
                Self([$sub_field::one(), $sub_field::zero()])
            }
        }
    };
}

pub use {const_extention_field_operation, extention_field_built_in};
