#[macro_export]
macro_rules! bls12_pairing {
    ($g2:ident, $pairng_coeff:ident, $pairing_affine:ident, $range_field:ident) => {
        use zero_crypto::behave::{G2Pairing, PairingRange, ParityCmp};

        impl PairingRange for $range_field {
            fn final_exp(self) -> Self {
                todo!()
            }
        }

        impl ParityCmp for $pairng_coeff {}

        impl G2Pairing for $g2 {
            type PairingRange = $range_field;

            type PairingCoeff = $pairng_coeff;

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

            fn add_eval(self) -> $range_field {
                todo!()
            }
        }
    };
}

pub use bls12_pairing;
