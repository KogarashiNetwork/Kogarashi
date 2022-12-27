#[macro_export]
macro_rules! bls12_pairing {
    ($g2:ident, $range_field:ident) => {
        use zero_crypto::behave::{G2Pairing, PairingRange};

        impl PairingRange for $range_field {
            fn final_exp(self) -> Self {
                todo!()
            }
        }

        impl G2Pairing for $g2 {
            type PairingRange = $range_field;

            fn double_eval(self) -> $range_field {
                todo!()
            }

            fn add_eval(self) -> $range_field {
                todo!()
            }
        }
    };
}

pub use bls12_pairing;
