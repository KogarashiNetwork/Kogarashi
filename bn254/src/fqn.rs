use crate::fq::Fq;
use crate::params::{
    BN_X, FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1, FROBENIUS_COEFF_FQ6_C1,
    FROBENIUS_COEFF_FQ6_C2,
};
use crate::{G1Affine, Gt, PairingCoeff};
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
impl Fq12 {
    // twisting isomorphism from E to E'
    pub(crate) fn untwist(self, coeffs: PairingCoeff, g1: G1Affine) -> Self {
        let mut c0 = coeffs.0;
        let mut c1 = coeffs.1;

        c0.0[0] *= g1.y;
        c0.0[1] *= g1.y;

        c1.0[0] *= g1.x;
        c1.0[1] *= g1.x;

        self.mul_by_034(c0, c1, coeffs.2)
    }

    pub fn final_exp(self) -> Gt {
        fn fp4_square(a: Fq2, b: Fq2) -> (Fq2, Fq2) {
            let t0 = a.square();
            let t1 = b.square();
            let mut t2 = t1.mul_by_nonres();
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
            t0 = t3.mul_by_nonres();
            z2 = t0 + z2;
            z2 = z2.double() + t0;

            z3 = t2 - z3;
            z3 = z3.double() + t2;

            Fq12([Fq6([z0, z4, z3]), Fq6([z2, z1, z5])])
        }

        #[must_use]
        fn cycolotomic_exp(f: Fq12) -> Fq12 {
            let mut res = Fq12::one();
            for is_one in (0..64).rev().map(|i| ((BN_X >> i) & 1) == 1) {
                res = cyclotomic_square(res);
                if is_one {
                    res *= f;
                }
            }
            res
        }

        let f = self;
        let f1 = f.conjugate();
        Gt(f.invert()
            .map(|mut f2| {
                f2 *= f1;
                let r = f2.frobenius_maps(2) * f2;

                let fp = r.frobenius_maps(1);
                let fp2 = r.frobenius_maps(2);
                let fp3 = fp2.frobenius_maps(1);

                let fu = cycolotomic_exp(r);
                let fu2 = cycolotomic_exp(fu);
                let fu3 = cycolotomic_exp(fu2);

                let y3 = fu.frobenius_maps(1).conjugate();

                let fu2p = fu2.frobenius_maps(1);
                let fu3p = fu3.frobenius_maps(1);

                let y2 = fu2.frobenius_maps(2);

                let y0 = fp * fp2 * fp3;
                let y1 = r.conjugate();
                let y5 = fu2.conjugate();

                let y4 = (fu * fu2p).conjugate();

                let mut y6 = cyclotomic_square((fu3 * fu3p).conjugate()) * y4 * y5;

                let mut t1 = y3 * y5 * y6;
                y6 *= y2;
                t1 = cyclotomic_square(cyclotomic_square(t1) * y6);

                let mut t0 = t1 * y1;
                t1 *= y0;
                t0 = cyclotomic_square(t0) * t1;
                t0
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
                        0xc556f62b2a98671d,
                        0x23a59ac167bcf363,
                        0x5ef208445f5f6f37,
                        0x12adf27ccb29382a,
                    ]),
                    Fq([
                        0x2e02a64acbd60549,
                        0xd618018ea58e4add,
                        0x14d585f1a45ba647,
                        0x1832226987c434fc,
                    ]),
                ]),
                Fq2([
                    Fq([
                        0x2306e4312363b991,
                        0x465f6072d4023bf4,
                        0xa2ff062a4a77e736,
                        0x76ea6f18435864a,
                    ]),
                    Fq([
                        0x172d1f257a4d598e,
                        0xddf5bc7b7ffb5ac0,
                        0xae0b22c0bbb0f602,
                        0x1B158F3C2FAE9B18,
                    ]),
                ]),
                Fq2([
                    Fq([
                        0x5cf9cc917da86724,
                        0xc799dc487a0b2753,
                        0xdf2027bf1de17a7,
                        0x197cda6cc3e20636,
                    ]),
                    Fq([
                        0xf16c96d081754cdb,
                        0xce0394312bceeb55,
                        0x644e4dcf1f01ff0a,
                        0xcbea85ee0b236cc,
                    ]),
                ]),
            ]),
            Fq6([
                Fq2([
                    Fq([
                        0x1bb0ce0def1b82a1,
                        0x4c4c9fe1cadefa95,
                        0x746d9990cb12b27e,
                        0x13495c08e5d415c5,
                    ]),
                    Fq([
                        0x9458abcb56d24998,
                        0xb17540bd2a9e5adb,
                        0x9a9983c82e401a9f,
                        0x1614817a84c16291,
                    ]),
                ]),
                Fq2([
                    Fq([
                        0x8975b68a2bab1f9c,
                        0x2fdd826b796e0f35,
                        0x6a90a35fa03dfaa5,
                        0x1ffef4581607fc37,
                    ]),
                    Fq([
                        0x7002907c28ebfe11,
                        0x7b0591d3d080da67,
                        0xde7e5aa2181f138e,
                        0x210e437dfc43d951,
                    ]),
                ]),
                Fq2([
                    Fq([
                        0x988ae2485b36cf53,
                        0x5091cc0581334e54,
                        0xda7903229312ca0f,
                        0x2a2341538eaee95c,
                    ]),
                    Fq([
                        0xd34bab373157aa84,
                        0x3511ed44fd0d8598,
                        0x67e42a0bc2ced972,
                        0x2b8f1d5dfd20c55b,
                    ]),
                ]),
            ]),
        ])
    }
}

impl Fq2 {
    pub const fn new_unchecked(val: [Fq; 2]) -> Self {
        Self(val)
    }
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
            0x4f082305b61f3f51,
            0x65e05aa45a1c72a3,
            0x6e14116da0605617,
            0x0c19139cb84c680a,
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
                    0x9e10460b6c3e7ea3,
                    0xcbc0b548b438e546,
                    0xdc2822db40c0ac2e,
                    0x183227397098d014,
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
    pub fn pow_vartime(&self, by: &[u64; 4]) -> Self {
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

    /// Multiply this element by quadratic nonresidue 9 + u.
    fn mul_by_nonres(self) -> Self {
        // (xi+y)(i+9) = (9x+y)i+(9y-x)
        let t0 = self.0[0];
        let t1 = self.0[1];
        // 8*x*i + 8*y
        let mut res = self.double().double().double();

        // 9*y - x
        res.0[0] += t0 - t1;
        // (9*x + y)i
        res.0[1] += t0 + t1;
        res
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
        let c0 = (self.0[1] * self.0[2]).mul_by_nonres();
        let c0 = self.0[0].square() - c0;

        let c1 = self.0[2].square().mul_by_nonres();
        let c1 = c1 - (self.0[0] * self.0[1]);

        let c2 = self.0[1].square();
        let c2 = c2 - (self.0[0] * self.0[2]);

        let tmp = ((self.0[1] * c2) + (self.0[2] * c1)).mul_by_nonres();
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
            t1 = t1.mul_by_nonres();
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
            t2 += c_c.mul_by_nonres();
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

        let c0 = s3.mul_by_nonres() + s0;
        let c1 = s4.mul_by_nonres() + s1;
        let c2 = s1 + s2 + s3 - s0 - s4;

        Self([c0, c1, c2])
    }

    fn mul_by_nonres(self) -> Self {
        Self([self.0[2].mul_by_nonres(), self.0[0], self.0[1]])
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
            (self.0[2] * c1).mul_by_nonres(),
            self.0[0] * c1,
            self.0[1] * c1,
        ])
    }

    pub fn mul_by_01(&self, c0: Fq2, c1: Fq2) -> Self {
        let a_a = self.0[0] * c0;
        let b_b = self.0[1] * c1;
        let t1 = ((self.0[1] + self.0[2]) * c1 - b_b).mul_by_nonres() + a_a;
        let t2 = (c0 + c1) * (self.0[0] + self.0[1]) - a_a - b_b;
        let t3 = (self.0[0] + self.0[2]) * c0 - a_a + b_b;

        Self([t1, t2, t3])
    }
}

impl Fq12 {
    fn get_invert(self) -> Option<Self> {
        (self.0[0].square() - self.0[1].square().mul_by_nonres())
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
        let c0 = bb.mul_by_nonres();
        let c0 = c0 + aa;

        Self([c0, c1])
    }

    fn square_ext_field(self) -> Self {
        let ab = self.0[0] * self.0[1];
        let c0c1 = self.0[0] + self.0[1];
        let c0 = self.0[1].mul_by_nonres() + self.0[0];
        let tmp = c0 * c0c1 - ab;

        Self([tmp - ab.mul_by_nonres(), ab.double()])
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

    pub fn mul_by_034(self, c0: Fq2, c3: Fq2, c4: Fq2) -> Self {
        let t0 = Fq6([
            self.0[0].0[0] * c0,
            self.0[0].0[1] * c0,
            self.0[0].0[2] * c0,
        ]);
        let mut t1 = self.0[1];
        t1 = t1.mul_by_01(c3, c4);
        let o = c0 + c3;
        let mut t2 = self.0[0] + self.0[1];
        t2 = t2.mul_by_01(o, c4);
        t2 -= t0;
        let b = t2 - t1;
        t1 = t1.mul_by_nonres();
        let a = t0 + t1;
        Self([a, b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;
    use zkstd::macros::field::field_test;

    field_test!(fq2_field, Fq2, 1000);
    field_test!(fq6_field, Fq6, 500);
    field_test!(fq12_field, Fq12, 100);

    #[test]
    fn fq2_mul_nonresidue_test() {
        let mut rng = OsRng;
        let b = Fq2([Fq::from(9), Fq::one()]);
        for _ in 0..1000 {
            let a = Fq2::random(&mut rng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonres(), expected)
        }
    }

    #[test]
    fn fq6_mul_nonresidue_test() {
        let mut rng = OsRng;
        let b = Fq6([Fq2::zero(), Fq2::one(), Fq2::zero()]);
        for _ in 0..1000 {
            let a = Fq6::random(&mut rng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonres(), expected)
        }
    }

    #[test]
    fn fq6_mul_by_1_test() {
        let mut rng = OsRng;
        for _ in 0..1000 {
            let c1 = Fq2::random(&mut rng);
            let a = Fq6::random(&mut rng);
            let b = Fq6([Fq2::zero(), c1, Fq2::zero()]);

            assert_eq!(a.mul_by_1(c1), a * b);
        }
    }

    #[test]
    fn fq6_mul_by_01_test() {
        let mut rng = OsRng;
        for _ in 0..1000 {
            let c0 = Fq2::random(&mut rng);
            let c1 = Fq2::random(&mut rng);
            let a = Fq6::random(&mut rng);
            let b = Fq6([c0, c1, Fq2::zero()]);

            assert_eq!(a.mul_by_01(c0, c1), a * b);
        }
    }

    #[test]
    fn test_fq12_mul_by_034() {
        let mut rng = OsRng;
        for _ in 0..1000 {
            let c0 = Fq2::random(&mut rng);
            let c3 = Fq2::random(&mut rng);
            let c4 = Fq2::random(&mut rng);
            let a = Fq12::random(&mut rng);
            let b = Fq12([
                Fq6([c0, Fq2::zero(), Fq2::zero()]),
                Fq6([c3, c4, Fq2::zero()]),
            ]);

            assert_eq!(a.mul_by_034(c0, c3, c4), a * b);
        }
    }

    #[test]
    fn fq12_frobenius_map_test() {
        let mut rng = OsRng;
        for _ in 0..1000 {
            let a = Fq12::random(&mut rng);

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
