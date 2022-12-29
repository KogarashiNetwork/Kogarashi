#[macro_export]
macro_rules! bls12_g2_pairing {
    ($g2:ident, $pairng_coeff:ident, $g2_pairing_affine:ident, $range_field:ident) => {
        use zero_crypto::behave::{G2Pairing, PairingRange, ParityCmp};

        impl ParityCmp for $pairng_coeff {}

        impl ParityCmp for $g2_pairing_affine {}

        impl G2Pairing for $g2 {
            type PairingRange = $range_field;

            type PairingCoeff = $pairng_coeff;

            type PairingRepr = $g2_pairing_affine;

            fn double_eval(mut self) -> $pairng_coeff {
                // Adaptation of Algorithm 26, https://eprint.iacr.org/2010/354.pdf
                let mut tmp0 = self.x;
                tmp0.square();

                let mut tmp1 = self.y;
                tmp1.square();

                let mut tmp2 = tmp1;
                tmp2.square();

                let mut tmp3 = tmp1;
                tmp3.add_assign(self.x);
                tmp3.square();
                tmp3.sub_assign(tmp0);
                tmp3.sub_assign(tmp2);
                tmp3.double();

                let mut tmp4 = tmp0;
                tmp4.double();
                tmp4.add_assign(tmp0);

                let mut tmp6 = self.x;
                tmp6.add_assign(tmp4);

                let mut tmp5 = tmp4;
                tmp5.square();

                let mut zsquared = self.z;
                zsquared.square();

                self.x = tmp5;
                self.x.sub_assign(tmp3);
                self.x.sub_assign(tmp3);

                self.z.add_assign(self.y);
                self.z.square();
                self.z.sub_assign(tmp1);
                self.z.sub_assign(zsquared);

                self.y = tmp3;
                self.y.sub_assign(self.x);
                self.y.mul_assign(tmp4);

                tmp2.double();
                tmp2.double();
                tmp2.double();

                self.y.sub_assign(tmp2);

                tmp3 = tmp4;
                tmp3.mul_assign(zsquared);
                tmp3.double();
                tmp3 = -tmp3;

                tmp6.square();
                tmp6.sub_assign(tmp0);
                tmp6.sub_assign(tmp5);

                tmp1.double();
                tmp1.double();

                tmp6.sub_assign(tmp1);

                tmp0 = self.z;
                tmp0.mul_assign(zsquared);
                tmp0.double();

                $pairng_coeff(tmp0, tmp3, tmp6)
            }

            fn add_eval(mut self, rhs: $g2) -> $pairng_coeff {
                // Adaptation of Algorithm 27, https://eprint.iacr.org/2010/354.pdf
                let mut zsquared = self.z;
                zsquared.square();

                let mut ysquared = rhs.y;
                ysquared.square();

                let mut t0 = zsquared;
                t0.mul_assign(rhs.x);

                let mut t1 = rhs.y;
                t1.add_assign(self.z);
                t1.square();
                t1.sub_assign(ysquared);
                t1.sub_assign(zsquared);
                t1.mul_assign(zsquared);

                let mut t2 = t0;
                t2.sub_assign(self.x);

                let mut t3 = t2;
                t3.square();

                let mut t4 = t3;
                t4.double();
                t4.double();

                let mut t5 = t4;
                t5.mul_assign(t2);

                let mut t6 = t1;
                t6.sub_assign(self.y);
                t6.sub_assign(self.y);

                let mut t9 = t6;
                t9.mul_assign(rhs.x);

                let mut t7 = t4;
                t7.mul_assign(self.x);

                self.x = t6;
                self.x.square();
                self.x.sub_assign(t5);
                self.x.sub_assign(t7);
                self.x.sub_assign(t7);

                self.z.add_assign(t2);
                self.z.square();
                self.z.sub_assign(zsquared);
                self.z.sub_assign(t3);

                let mut t10 = rhs.y;
                t10.add_assign(self.z);

                let mut t8 = t7;
                t8.sub_assign(self.x);
                t8.mul_assign(t6);

                t0 = self.y;
                t0.mul_assign(t5);
                t0.double();

                self.y = t8;
                self.y.sub_assign(t0);

                t10.square();
                t10.sub_assign(ysquared);

                let mut ztsquared = self.z;
                ztsquared.square();

                t10.sub_assign(ztsquared);

                t9.double();
                t9.sub_assign(t10);

                t10 = self.z;
                t10.double();

                t6 = -t6;

                t1 = t6;
                t1.double();

                $pairng_coeff(t10, t1, t9)
            }
        }

        impl Default for $g2_pairing_affine {
            fn default() -> Self {
                $g2_pairing_affine {
                    coeffs: vec![],
                    infinity: true,
                }
            }
        }

        impl From<$g2> for $g2_pairing_affine {
            fn from(a: $g2) -> $g2_pairing_affine {
                if a.is_identity() {
                    $g2_pairing_affine::default()
                } else {
                    let mut coeffs = vec![];
                    let mut acc = a;

                    let mut found_one = false;
                    for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
                        if !found_one {
                            found_one = i;
                            continue;
                        }

                        coeffs.push(acc.double_eval());

                        if i {
                            coeffs.push(acc.add_eval(a));
                        }
                    }

                    coeffs.push(acc.double_eval());

                    $g2_pairing_affine {
                        coeffs,
                        infinity: false,
                    }
                }
            }
        }
    };
}

pub use bls12_g2_pairing;
