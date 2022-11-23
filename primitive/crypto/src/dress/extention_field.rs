#[macro_export]
macro_rules! extention_field_operation {
    ($extention_field:ident, $sub_field:ident) => {
        extention_field_built_in!($extention_field);

        impl ExtentionField for $extention_field {}

        impl $extention_field {
            pub const fn zero() -> $extention_field {
                $extention_field([$sub_field::zero(), $sub_field::zero()])
            }

            pub const fn one() -> $extention_field {
                $extention_field([$sub_field::one(), $sub_field::zero()])
            }
        }
    };
}

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

pub use {extention_field_built_in, extention_field_operation};
