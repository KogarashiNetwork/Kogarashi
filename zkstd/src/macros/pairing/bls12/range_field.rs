#[macro_export]
macro_rules! bls12_range_field_pairing {
    ($range_field:ident, $quadratic_field:ident, $gt:ident, $g1_affine:ident, $pairng_coeff:ident, $bls_x:ident, $bls_x_is_negative:ident) => {
        impl PairingRange for $range_field {
            type G1Affine = $g1_affine;
            type G2Coeff = $pairng_coeff;
            type QuadraticField = $quadratic_field;
            type Gt = $gt;

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
                let c0 = bb.mul_by_nonresidue();
                let c0 = c0 + aa;

                Self([c0, c1])
            }

            fn final_exp(self) -> Self::Gt {
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
                    z0 = z0.double() + t0;

                    z1 = t1 + z1;
                    z1 = z1.double() + t1;

                    let (mut t0, t1) = fp4_square(z2, z3);
                    let (t2, t3) = fp4_square(z4, z5);

                    // For C
                    z4 = t0 - z4;
                    z4 = z4.double() + t0;

                    z5 = t1 + z5;
                    z5 = z5.double() + t1;

                    // For B
                    t0 = t3.mul_by_nonresidue();
                    z2 = t0 + z2;
                    z2 = z2.double() + t0;

                    z3 = t2 - z3;
                    z3 = z3.double() + t2;

                    Fq12([Fq6([z0, z4, z3]), Fq6([z2, z1, z5])])
                }

                #[must_use]
                fn cycolotomic_exp(f: Fq12) -> Fq12 {
                    let mut tmp = Fq12::one();
                    let mut found_one = false;
                    for i in (0..64).rev().map(|b| ((BLS_X >> b) & 1) == 1) {
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
                let mut t0 = f.frobenius_maps(6);
                $gt(f
                    .invert()
                    .map(|mut t1| {
                        let mut t2 = t0 * t1;
                        t1 = t2;
                        t2 = t2.frobenius_maps(2);
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
                        t1 = t1.frobenius_maps(3);
                        t6 *= t5;
                        t6 = t6.frobenius_map();
                        t3 *= t0;
                        t3 = t3.frobenius_maps(2);
                        t3 *= t1;
                        t3 *= t6;
                        f = t3 * t4;

                        f
                    })
                    .unwrap())
            }
        }

        impl $range_field {
            pub const fn generator() -> Self {
                Fq12([
                    Fq6([
                        Fq2([
                            Fq([
                                0x1972e433a01f85c5,
                                0x97d32b76fd772538,
                                0xc8ce546fc96bcdf9,
                                0xcef63e7366d40614,
                                0xa611342781843780,
                                0x13f3448a3fc6d825,
                            ]),
                            Fq([
                                0xd26331b02e9d6995,
                                0x9d68a482f7797e7d,
                                0x9c9b29248d39ea92,
                                0xf4801ca2e13107aa,
                                0xa16c0732bdbcb066,
                                0x083ca4afba360478,
                            ]),
                        ]),
                        Fq2([
                            Fq([
                                0x59e261db0916b641,
                                0x2716b6f4b23e960d,
                                0xc8e55b10a0bd9c45,
                                0x0bdb0bd99c4deda8,
                                0x8cf89ebf57fdaac5,
                                0x12d6b7929e777a5e,
                            ]),
                            Fq([
                                0x5fc85188b0e15f35,
                                0x34a06e3a8f096365,
                                0xdb3126a6e02ad62c,
                                0xfc6f5aa97d9a990b,
                                0xa12f55f5eb89c210,
                                0x1723703a926f8889,
                            ]),
                        ]),
                        Fq2([
                            Fq([
                                0x93588f2971828778,
                                0x43f65b8611ab7585,
                                0x3183aaf5ec279fdf,
                                0xfa73d7e18ac99df6,
                                0x64e176a6a64c99b0,
                                0x179fa78c58388f1f,
                            ]),
                            Fq([
                                0x672a0a11ca2aef12,
                                0x0d11b9b52aa3f16b,
                                0xa44412d0699d056e,
                                0xc01d0177221a5ba5,
                                0x66e0cede6c735529,
                                0x05f5a71e9fddc339,
                            ]),
                        ]),
                    ]),
                    Fq6([
                        Fq2([
                            Fq([
                                0xd30a88a1b062c679,
                                0x5ac56a5d35fc8304,
                                0xd0c834a6a81f290d,
                                0xcd5430c2da3707c7,
                                0xf0c27ff780500af0,
                                0x09245da6e2d72eae,
                            ]),
                            Fq([
                                0x9f2e0676791b5156,
                                0xe2d1c8234918fe13,
                                0x4c9e459f3c561bf4,
                                0xa3e85e53b9d3e3c1,
                                0x820a121e21a70020,
                                0x15af618341c59acc,
                            ]),
                        ]),
                        Fq2([
                            Fq([
                                0x7c95658c24993ab1,
                                0x73eb38721ca886b9,
                                0x5256d749477434bc,
                                0x8ba41902ea504a8b,
                                0x04a3d3f80c86ce6d,
                                0x18a64a87fb686eaa,
                            ]),
                            Fq([
                                0xbb83e71bb920cf26,
                                0x2a5277ac92a73945,
                                0xfc0ee59f94f046a0,
                                0x7158cdf3786058f7,
                                0x7cc1061b82f945f6,
                                0x03f847aa9fdbe567,
                            ]),
                        ]),
                        Fq2([
                            Fq([
                                0x8078dba56134e657,
                                0x1cd7ec9a43998a6e,
                                0xb1aa599a1a993766,
                                0xc9a0f62f0842ee44,
                                0x8e159be3b605dffa,
                                0x0c86ba0d4af13fc2,
                            ]),
                            Fq([
                                0xe80ff2a06a52ffb1,
                                0x7694ca48721a906c,
                                0x7583183e03b08514,
                                0xf567afdd40cee4e2,
                                0x9a6d96d2e526a5fc,
                                0x197e9f49861f2242,
                            ]),
                        ]),
                    ]),
                ])
            }
        }
    };
}

pub use bls12_range_field_pairing;
