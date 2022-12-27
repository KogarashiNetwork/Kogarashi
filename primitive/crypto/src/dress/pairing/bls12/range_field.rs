#[macro_export]
macro_rules! bls12_range_field_pairing {
    ($range_field:ident, $g1_affine:ident, $pairng_coeff:ident) => {
        impl PairingRange for $range_field {
            type G1Affine = $g1_affine;

            type G2Coeff = $pairng_coeff;

            fn final_exp(self) -> Self {
                todo!()
            }

            fn conjugate(self) -> Self {
                todo!()
            }

            fn untwist(self, coeffs: Self::G2Coeff, g1: Self::G1Affine) -> Self {
                todo!()
            }
        }
    };
}

pub use bls12_range_field_pairing;
