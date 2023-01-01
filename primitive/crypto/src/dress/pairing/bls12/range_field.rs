#[macro_export]
macro_rules! bls12_range_field_pairing {
    ($range_field:ident, $quadratic_field:ident, $g1_affine:ident, $pairng_coeff:ident, $bls_x:ident, $bls_x_is_negative:ident) => {
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

                self.mul_by_014(coeffs.2, c1, c0)
            }

            fn mul_by_014(
                mut self,
                c0: Self::QuadraticField,
                c1: Self::QuadraticField,
                c4: Self::QuadraticField,
            ) -> Self {
                let aa = self.0[0].mul_by_01(c0, c1);
                let bb = self.0[1].mul_by_1(c4);
                let o = c1 + c4;
                let c1 = self.0[1] + self.0[0];
                let c1 = c1.mul_by_01(c0, o);
                let c1 = c1 - aa - bb;
                let c0 = bb;
                let c0 = c0.mul_by_nonresidue();
                let c0 = c0 + aa;

                Self([c0, c1])
            }

            fn final_exp(self) -> Option<Self> {
                let mut f1 = self;
                f1.conjugate();

                match self.invert() {
                    Some(mut f2) => {
                        let mut r = f1;
                        r *= f2;
                        f2 = r.frobenius_map(2);
                        r *= f2;

                        let mut x = $bls_x;
                        let y0 = r.square();
                        let mut y1 = y0.pow(x);
                        x >>= 1;
                        let mut y2 = y1.pow(x);
                        x <<= 1;
                        let mut y3 = r.conjugate();
                        y1 *= y3.conjugate();
                        y1 *= y2;
                        y2 = y1;
                        y2 = y2.pow(x);
                        y3 = y2;
                        y3 = y3.pow(x);
                        y1 = y1.conjugate();
                        y3 *= y1;
                        y1 = y1.conjugate();
                        y1 = y1.frobenius_map(3);
                        y2 = y2.frobenius_map(2);
                        y1 *= y2;
                        y2 = y3;
                        y2 = y2.pow(x);
                        y2 *= y0;
                        y2 *= r;
                        y1 *= y2;
                        y2 = y3;
                        y2 = y2.frobenius_map(1);
                        y1 *= y2;

                        Some(y1)
                    }
                    None => None,
                }
            }
        }
    };
}

pub use bls12_range_field_pairing;
