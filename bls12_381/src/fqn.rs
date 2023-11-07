use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use crate::gt::Gt;
use crate::params::{
    BLS_X, FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1, FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ6_C2,
};
use zkstd::common::*;
use zkstd::macros::extension_field::*;

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENSION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENSION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENSION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENSION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
impl PairingRange for Fq12 {
    type G1Affine = G1Affine;
    type G2Coeff = PairingCoeff;
    type QuadraticField = Fq2;
    type Gt = Gt;

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
        self,
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
        Gt(f.invert()
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

impl Fq12 {
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

impl Fq2 {
    pub(crate) const fn add_const(self, rhs: Self) -> Self {
        Self([self.0[0].add_const(rhs.0[0]), self.0[1].add_const(rhs.0[1])])
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    #[inline]
    pub fn lexicographically_largest(&self) -> bool {
        // If this element's c1 coefficient is lexicographically largest
        // then it is lexicographically largest. Otherwise, in the event
        // the c1 coefficient is zero and the c0 coefficient is
        // lexicographically largest, then this element is lexicographically
        // largest.

        self.0[1].lexicographically_largest()
            | self.0[1].is_zero() & self.0[0].lexicographically_largest()
    }

    pub fn sqrt(&self) -> Option<Self> {
        // Algorithm 9, https://eprint.iacr.org/2012/685.pdf
        // with constant time modifications.

        if self.is_zero() {
            return Some(Fq2::zero());
        }

        // a1 = self^((p - 3) / 4)
        let a1 = self.pow_vartime(&[
            0xee7fbfffffffeaaa,
            0x7aaffffac54ffff,
            0xd9cc34a83dac3d89,
            0xd91dd2e13ce144af,
            0x92c6e9ed90d2eb35,
            0x680447a8e5ff9a6,
        ]);
        // alpha = a1^2 * self = self^((p - 3) / 2 + 1) = self^((p - 1) / 2)
        let alpha = a1.square() * *self;
        // x0 = self^((p + 1) / 4)
        let x0 = a1 * *self;

        // In the event that alpha = -1, the element is order p - 1 and so
        // we're just trying to get the square of an element of the subfield
        // Fp. This is given by x0 * u, since u = sqrt(-1). Since the element
        // x0 = a + bu has b = 0, the solution is therefore au.
        let sqrt = match alpha == Fq2::one().neg() {
            true => Fq2([-x0.0[1], x0.0[0]]),
            false => {
                (alpha + Fq2::one()).pow_vartime(&[
                    0xdcff7fffffffd555,
                    0xf55ffff58a9ffff,
                    0xb39869507b587b12,
                    0xb23ba5c279c2895f,
                    0x258dd3db21a5d66b,
                    0xd0088f51cbff34d,
                ]) * x0
            }
        };
        match sqrt.square() == *self {
            true => Some(sqrt),
            false => None,
        }
    }

    /// Although this is labeled "vartime", it is only
    /// variable time with respect to the exponent. It
    /// is also not exposed in the public API.
    pub fn pow_vartime(&self, by: &[u64; 6]) -> Self {
        let mut res = Self::one();
        for e in by.iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res *= *self;
                }
            }
        }
        res
    }
}

impl Debug for Fq2 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?} + {:?} u", self.0[0], self.0[1])
    }
}

impl Debug for Fq6 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:?} + ({:?}) v + ({:?}) v^2",
            self.0[0], self.0[1], self.0[2]
        )
    }
}

impl Debug for Fq12 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?} + ({:?}) w", self.0[0], self.0[1])
    }
}

impl Fq2 {
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
        let re = self.0[0].square() - self.0[1].square();
        let im = (self.0[0] * self.0[1]).double();
        Self([re, im])
    }

    fn mul_by_nonres(self) -> Self {
        Self([self.0[0] - self.0[1], self.0[0] + self.0[1]])
    }

    fn conjugate(&self) -> Self {
        Self([self.0[0], -self.0[1]])
    }

    pub fn frobenius_map(&self) -> Self {
        self.conjugate()
    }

    fn frobenius_maps(self, power: usize) -> Self {
        let c0 = self.0[0];
        let c1 = self.0[1] * FROBENIUS_COEFF_FQ2_C1[power % 2];

        Self([c0, c1])
    }
}

impl Fq6 {
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
        let a_a = self.0[0] * rhs.0[0];
        let b_b = self.0[1] * rhs.0[1];
        let c_c = self.0[2] * rhs.0[2];

        let mut t1 = rhs.0[1] + rhs.0[2];
        {
            let tmp = self.0[1] + self.0[2];

            t1 *= tmp;
            t1 -= b_b;
            t1 -= c_c;
            t1 = t1.mul_by_nonresidue();
            t1 += a_a;
        }

        let mut t3 = rhs.0[0] + rhs.0[2];
        {
            let tmp = self.0[0] + self.0[2];

            t3 *= tmp;
            t3 -= a_a;
            t3 += b_b;
            t3 -= c_c;
        }

        let mut t2 = rhs.0[0] + rhs.0[1];
        {
            let tmp = self.0[0] + self.0[1];

            t2 *= tmp;
            t2 -= a_a;
            t2 -= b_b;
            t2 += c_c.mul_by_nonresidue();
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

        let c0 = s3.mul_by_nonresidue() + s0;
        let c1 = s4.mul_by_nonresidue() + s1;
        let c2 = s1 + s2 + s3 - s0 - s4;

        Self([c0, c1, c2])
    }

    fn mul_by_nonres(self) -> Self {
        Self([self.0[2].mul_by_nonresidue(), self.0[0], self.0[1]])
    }

    pub fn frobenius_map(&self) -> Self {
        let c0 = self.0[0].frobenius_map();
        let c1 = self.0[1].frobenius_map() * FROBENIUS_COEFF_FQ6_C1[1];
        let c2 = self.0[2].frobenius_map() * FROBENIUS_COEFF_FQ6_C2[1];

        Fq6([c0, c1, c2])
    }

    fn frobenius_maps(self, power: usize) -> Self {
        let c0 = self.0[0].frobenius_maps(power);
        let c1 = self.0[1].frobenius_maps(power) * FROBENIUS_COEFF_FQ6_C1[power % 6];
        let c2 = self.0[2].frobenius_maps(power) * FROBENIUS_COEFF_FQ6_C2[power % 6];

        Self([c0, c1, c2])
    }

    pub fn mul_by_1(&self, c1: Fq2) -> Self {
        Self([
            (self.0[2] * c1).mul_by_nonresidue(),
            self.0[0] * c1,
            self.0[1] * c1,
        ])
    }

    pub fn mul_by_01(&self, c0: Fq2, c1: Fq2) -> Self {
        let a_a = self.0[0] * c0;
        let b_b = self.0[1] * c1;
        let t1 = (self.0[2] * c1).mul_by_nonresidue() + a_a;
        let t2 = (c0 + c1) * (self.0[0] + self.0[1]) - a_a - b_b;
        let t3 = self.0[2] * c0 + b_b;

        Self([t1, t2, t3])
    }
}

impl Fq12 {
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

    pub fn frobenius_map(self) -> Self {
        let c0 = self.0[0].frobenius_map();
        let c1 =
            self.0[1].frobenius_map() * Fq6([FROBENIUS_COEFF_FQ12_C1[1], Fq2::zero(), Fq2::zero()]);

        Self([c0, c1])
    }

    fn frobenius_maps(self, power: usize) -> Self {
        let c0 = self.0[0].frobenius_maps(power);
        let c1 = self.0[1].frobenius_maps(power);
        let c1 = Fq6([
            c1.0[0] * FROBENIUS_COEFF_FQ12_C1[power % 12],
            c1.0[1] * FROBENIUS_COEFF_FQ12_C1[power % 12],
            c1.0[2] * FROBENIUS_COEFF_FQ12_C1[power % 12],
        ]);

        Self([c0, c1])
    }

    fn mul_by_014(self, c0: Fq2, c1: Fq2, c4: Fq2) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use zkstd::macros::field::field_test;

    field_test!(fq2_field, Fq2, 1000);
    field_test!(fq6_field, Fq6, 500);
    field_test!(fq12_field, Fq12, 100);

    #[test]
    fn fq2_mul_nonresidue_test() {
        let b = Fq2([Fq::one(); 2]);
        for _ in 0..1000 {
            let a = Fq2::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }

    #[test]
    fn fq6_mul_nonresidue_test() {
        let b = Fq6([Fq2::zero(), Fq2::one(), Fq2::zero()]);
        for _ in 0..1000 {
            let a = Fq6::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonresidue(), expected)
        }
    }

    #[test]
    fn fq6_mul_by_1_test() {
        for _ in 0..1000 {
            let c1 = Fq2::random(OsRng);
            let a = Fq6::random(OsRng);
            let b = Fq6([Fq2::zero(), c1, Fq2::zero()]);

            assert_eq!(a.mul_by_1(c1), a * b);
        }
    }

    #[test]
    fn fq6_mul_by_01_test() {
        for _ in 0..1000 {
            let c0 = Fq2::random(OsRng);
            let c1 = Fq2::random(OsRng);
            let a = Fq6::random(OsRng);
            let b = Fq6([c0, c1, Fq2::zero()]);

            assert_eq!(a.mul_by_01(c0, c1), a * b);
        }
    }

    #[test]
    fn fq12_mul_by_014_test() {
        for _ in 0..1000 {
            let c0 = Fq2::random(OsRng);
            let c1 = Fq2::random(OsRng);
            let c5 = Fq2::random(OsRng);
            let a = Fq12::random(OsRng);
            let b = Fq12([
                Fq6([c0, c1, Fq2::zero()]),
                Fq6([Fq2::zero(), c5, Fq2::zero()]),
            ]);

            assert_eq!(a.mul_by_014(c0, c1, c5), a * b);
        }
    }

    #[test]
    fn fq12_frobenius_map_test() {
        for _ in 0..1000 {
            let a = Fq12::random(OsRng);

            for i in 0..12 {
                let mut b = a;
                for _ in 0..i {
                    b = b.frobenius_map();
                }
                assert_eq!(a.frobenius_maps(i), b);
            }

            assert_eq!(a, a.frobenius_maps(12));
            assert_eq!(
                a,
                a.frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
                    .frobenius_map()
            );
        }
    }
}
