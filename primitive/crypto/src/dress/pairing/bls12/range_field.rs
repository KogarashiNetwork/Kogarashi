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
                #[must_use]
                fn fp4_square(a: Fq2, b: Fq2) -> (Fq2, Fq2) {
                    let t0 = a.square();
                    let t1 = b.square();
                    let mut t2 = t1.mul_by_nonresidue();
                    let c0 = t2 + t0;
                    t2 = a + b;
                    t2 = t2.square();
                    t2 -= t0;
                    let c1 = t2 - t1;

                    (c0, c1)
                }
                // Adaptation of Algorithm 5.5.4, Guide to Pairing-Based Cryptography
                // Faster Squaring in the Cyclotomic Subgroup of Sixth Degree Extensions
                // https://eprint.iacr.org/2009/565.pdf
                #[must_use]
                fn cyclotomic_square(f: Fq12) -> Fq12 {
                    let mut z0 = f.0[0].0[0];
                    let mut z4 = f.0[0].0[1];
                    let mut z3 = f.0[0].0[2];
                    let mut z2 = f.0[1].0[0];
                    let mut z1 = f.0[1].0[1];
                    let mut z5 = f.0[1].0[2];

                    let (t0, t1) = fp4_square(z0, z1);

                    // For A
                    z0 = t0 - z0;
                    z0 = z0 + z0 + t0;

                    z1 = t1 + z1;
                    z1 = z1 + z1 + t1;

                    let (mut t0, t1) = fp4_square(z2, z3);
                    let (t2, t3) = fp4_square(z4, z5);

                    // For C
                    z4 = t0 - z4;
                    z4 = z4 + z4 + t0;

                    z5 = t1 + z5;
                    z5 = z5 + z5 + t1;

                    // For B
                    t0 = t3.mul_by_nonresidue();
                    z2 = t0 + z2;
                    z2 = z2 + z2 + t0;

                    z3 = t2 - z3;
                    z3 = z3 + z3 + t2;

                    Fq12([Fq6([z0, z4, z3]), Fq6([z2, z1, z5])])
                }
                #[must_use]
                fn cycolotomic_exp(f: Fq12) -> Fq12 {
                    let x = BLS_X;
                    let mut tmp = Fq12::one();
                    let mut found_one = false;
                    for i in (0..64).rev().map(|b| ((x >> b) & 1) == 1) {
                        if found_one {
                            tmp = cyclotomic_square(tmp)
                        } else {
                            found_one = i;
                        }

                        if i {
                            tmp *= f;
                        }
                    }

                    tmp.conjugate()
                }

                let mut f = self;
                let mut t0 = f
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map();
                f.invert().map(|mut t1| {
                    let mut t2 = t0 * t1;
                    t1 = t2;
                    t2 = t2.frobenius_map().frobenius_map();
                    t2 *= t1;
                    t1 = cyclotomic_square(t2).conjugate();
                    let mut t3 = cycolotomic_exp(t2);
                    let mut t4 = cyclotomic_square(t3);
                    let mut t5 = t1 * t3;
                    t1 = cycolotomic_exp(t5);
                    t0 = cycolotomic_exp(t1);
                    let mut t6 = cycolotomic_exp(t0);
                    t6 *= t4;
                    t4 = cycolotomic_exp(t6);
                    t5 = t5.conjugate();
                    t4 *= t5 * t2;
                    t5 = t2.conjugate();
                    t1 *= t2;
                    t1 = t1.frobenius_map().frobenius_map().frobenius_map();
                    t6 *= t5;
                    t6 = t6.frobenius_map();
                    t3 *= t0;
                    t3 = t3.frobenius_map().frobenius_map();
                    t3 *= t1;
                    t3 *= t6;
                    f = t3 * t4;

                    f
                })
            }
        }
    };
}

pub use bls12_range_field_pairing;
