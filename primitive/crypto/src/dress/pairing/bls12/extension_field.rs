#[macro_export]
macro_rules! peculiar_extension_field_operation {
    ($fq2:ident, $fq6:ident, $fq12:ident, $frobenius_coeff_fq2_c1:ident, $frobenius_coeff_fq6_c1:ident, $frobenius_coeff_fq12_c1:ident, $bls_x_is_negative:ident) => {
        impl $fq2 {
            fn get_invert(self) -> Option<Self> {
                match self.is_zero() {
                    true => None,
                    _ => {
                        let t = self.0[0].square() + self.0[1].square();
                        let t_inv = t.invert().unwrap();
                        Some(Self([t_inv * self.0[0], t_inv * -self.0[1]]))
                    }
                }
            }

            fn mul_ext_field(self, rhs: Self) -> Self {
                let re = (self.0[0] * rhs.0[0]) - (self.0[1] * rhs.0[1]);
                let im = (self.0[0] * rhs.0[1]) + (self.0[1] * rhs.0[0]);
                Self([re, im])
            }

            fn square_ext_field(self) -> Self {
                self * self
            }

            fn mul_by_nonres(self) -> Self {
                Self([self.0[0] - self.0[1], self.0[0] + self.0[1]])
            }

            fn frobenius_map(self, power: usize) -> Self {
                Self([self.0[0], self.0[1] * $frobenius_coeff_fq2_c1[power % 2]])
            }
        }

        impl $fq6 {
            fn get_invert(self) -> Option<Self> {
                let c0 = (self.0[1] * self.0[2]).mul_by_nonresidue();
                let c0 = self.0[0].square() - c0;

                let c1 = self.0[2].square().mul_by_nonresidue();
                let c1 = c1 - (self.0[0] * self.0[1]);

                let c2 = self.0[1].square();
                let c2 = c2 - (self.0[0] * self.0[2]);

                let tmp = ((self.0[1] * c2) + (self.0[2] * c1)).mul_by_nonresidue();
                let tmp = tmp + (self.0[0] * c0);

                tmp.invert().map(|t| Self([t * c0, t * c1, t * c2]))
            }

            fn mul_ext_field(self, rhs: Self) -> Self {
                let mut a_a = self.0[0];
                let mut b_b = self.0[1];
                let mut c_c = self.0[2];
                a_a *= rhs.0[0];
                b_b *= rhs.0[1];
                c_c *= rhs.0[2];

                let mut t1 = rhs.0[1];
                t1 += rhs.0[2];
                {
                    let mut tmp = self.0[1];
                    tmp += self.0[2];

                    t1 *= tmp;
                    t1 -= b_b;
                    t1 -= c_c;
                    t1 = t1.mul_by_nonresidue();
                    t1 += a_a;
                }

                let mut t3 = rhs.0[0];
                t3 += rhs.0[2];
                {
                    let mut tmp = self.0[0];
                    tmp += self.0[2];

                    t3 *= tmp;
                    t3 -= a_a;
                    t3 += b_b;
                    t3 -= c_c;
                }

                let mut t2 = rhs.0[0];
                t2 += rhs.0[1];
                {
                    let mut tmp = self.0[0];
                    tmp += self.0[1];

                    t2 *= tmp;
                    t2 -= a_a;
                    t2 -= b_b;
                    c_c = c_c.mul_by_nonresidue();
                    t2 += c_c;
                }

                Self([t1, t2, t3])
            }

            fn square_ext_field(self) -> Self {
                let s0 = self.0[0].square();
                let ab = self.0[0] * self.0[1];
                let s1 = ab.double();
                let mut s2 = self.0[0];
                s2 -= self.0[1];
                s2 += self.0[2];
                s2 = s2.square();
                let bc = self.0[1] * self.0[2];
                let s3 = bc.double();
                let s4 = self.0[2].square();

                let tmp1 = s3.mul_by_nonresidue() + s0;
                let tmp2 = s4.mul_by_nonresidue() + s1;
                let tmp3 = s1 + s2 + s3 - s0 - s4;

                Self([tmp1, tmp2, tmp3])
            }

            fn mul_by_nonres(self) -> Self {
                Self([self.0[2].mul_by_nonresidue(), self.0[0], self.0[1]])
            }

            fn frobenius_map(self, power: usize) -> Self {
                self.0[0].frobenius_map(power);
                self.0[1].frobenius_map(power);
                self.0[2].frobenius_map(power);

                Self([
                    self.0[0],
                    self.0[1] * $frobenius_coeff_fq6_c1[power % 6],
                    self.0[2] * $frobenius_coeff_fq6_c1[power % 6],
                ])
            }

            pub fn mul_by_1(&self, c1: $fq2) -> Self {
                Self([
                    (self.0[2] * c1).mul_by_nonresidue(),
                    self.0[0] * c1,
                    self.0[1] * c1,
                ])
            }

            pub fn mul_by_01(&self, c0: $fq2, c1: $fq2) -> Self {
                let a_a = self.0[0] * c0;
                let b_b = self.0[1] * c1;
                let t1 = (self.0[2] * c1).mul_by_nonresidue() + a_a;
                let t2 = (c0 + c1) * (self.0[0] + self.0[1]) - a_a - b_b;
                let t3 = self.0[2] * c0 + b_b;

                Self([t1, t2, t3])
            }
        }

        impl $fq12 {
            fn get_invert(self) -> Option<Self> {
                (self.0[0].square() - self.0[1].square().mul_by_nonresidue())
                    .invert()
                    .map(|t| Self([self.0[0] * t, self.0[1] * -t]))
            }

            fn mul_ext_field(self, rhs: Self) -> Self {
                let aa = self.0[0] * rhs.0[0];
                let bb = self.0[1] * rhs.0[1];
                let o = rhs.0[0] + rhs.0[1];
                let c1 = self.0[1] + self.0[0];
                let c1 = c1 * o;
                let c1 = c1 - aa;
                let c1 = c1 - bb;
                let c0 = bb.mul_by_nonresidue();
                let c0 = c0 + aa;

                Self([c0, c1])
            }

            fn square_ext_field(self) -> Self {
                let ab = self.0[0] * self.0[1];
                let c0c1 = self.0[0] + self.0[1];
                let c0 = self.0[1].mul_by_nonresidue() + self.0[0];
                let tmp = c0 * c0c1 - ab;

                Self([tmp - ab.mul_by_nonresidue(), ab.double()])
            }

            fn mul_by_nonres(self) -> Self {
                unimplemented!()
            }

            pub fn conjugate(self) -> Self {
                Self([self.0[0], -self.0[1]])
            }

            fn frobenius_map(self, power: usize) -> Self {
                self.0[0].frobenius_map(power);
                self.0[1].frobenius_map(power);

                Self([
                    self.0[0],
                    $fq6([
                        self.0[1].0[0] * $frobenius_coeff_fq12_c1[power % 12],
                        self.0[1].0[1] * $frobenius_coeff_fq12_c1[power % 12],
                        self.0[1].0[2] * $frobenius_coeff_fq12_c1[power % 12],
                    ]),
                ])
            }

            fn pow(self, exp: u64) -> Self {
                let mut res = Self::one();

                let mut found_one = false;
                for i in (0..64).rev().map(|b| (((exp >> 1) >> b) & 1) == 1) {
                    if found_one {
                        res = res.square();
                    } else {
                        found_one = i.into();
                    }

                    if i.into() {
                        res = res.square();
                    }
                }

                if $bls_x_is_negative {
                    res.conjugate()
                } else {
                    res
                }
            }

            fn mul_by_014(self, c0: $fq2, c1: $fq2, c4: $fq2) -> Self {
                let aa = self.0[0].mul_by_01(c0, c1);
                let bb = self.0[1].mul_by_1(c4);
                let o = c1 + c4;
                let c1 = self.0[1] + self.0[0];
                let c1 = c1.mul_by_01(c0, o);
                let c0 = bb;
                let c0 = c0.mul_by_nonresidue();

                Self([c0 + aa, c1 - aa - bb])
            }
        }
    };
}

pub use peculiar_extension_field_operation;
