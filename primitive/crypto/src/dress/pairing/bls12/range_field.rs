#[macro_export]
macro_rules! bls12_range_field_pairing {
    ($range_field:ident, $quadratic_field:ident, $g1_affine:ident, $pairng_coeff:ident) => {
        impl PairingRange for $range_field {
            type G1Affine = $g1_affine;

            type G2Coeff = $pairng_coeff;

            type QuadraticField = $quadratic_field;

            // twisting isomorphism from E to E'
            fn untwist(self, coeffs: Self::G2Coeff, g1: Self::G1Affine) -> Self {
                let mut c0 = coeffs.0;
                let mut c1 = coeffs.1;

                c0.0[0] *= g1.y;
                c0.0[1] *= g1.y;

                c1.0[0] *= g1.x;
                c1.0[1] *= g1.x;

                Self::default()
            }

            fn mul_by_014(
                mut self,
                c0: Self::QuadraticField,
                c1: Self::QuadraticField,
                c4: Self::QuadraticField,
            ) -> Self {
                todo!()
            }

            fn conjugate(self) -> Self {
                todo!()
            }

            fn final_exp(self) -> Self {
                todo!()
            }
        }
    };
}

pub use bls12_range_field_pairing;
