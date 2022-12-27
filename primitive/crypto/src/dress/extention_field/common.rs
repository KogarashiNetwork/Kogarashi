#[macro_export]
macro_rules! extention_field_built_in {
    ($extention_field:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ExtentionField for $extention_field {}

        impl ParityCmp for $extention_field {}

        impl Basic for $extention_field {}

        impl Default for $extention_field {
            fn default() -> Self {
                Self::zero()
            }
        }

        impl Display for $extention_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                for i in self.0.iter().rev() {
                    write!(f, "0x")?;
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }

        impl LowerHex for $extention_field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                for i in self.0.iter().rev() {
                    write!(f, "0x")?;
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! const_extention_field_operation {
    ($extention_field:ident, $sub_field:ident, $limbs_length:ident) => {
        impl $extention_field {
            pub const fn zero() -> Self {
                Self([$sub_field::zero(); $limbs_length])
            }

            pub const fn one() -> Self {
                let mut limbs = [$sub_field::zero(); $limbs_length];
                limbs[0] = $sub_field::one();
                Self(limbs)
            }

            pub const fn dummy() -> Self {
                unimplemented!()
            }
        }
    };
}

#[macro_export]
macro_rules! construct_extention_field {
    ($extention_field:ident, $sub_field:ident, $limbs_length:ident) => {
        #[derive(Debug, Clone, Copy, Decode, Encode)]
        pub struct $extention_field(pub(crate) [$sub_field; $limbs_length]);
    };
}

pub use {const_extention_field_operation, construct_extention_field, extention_field_built_in};
