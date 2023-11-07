use crate::fq::Fq;
use crate::params::{
    FROBENIUS_COEFF_FQ12_C1, FROBENIUS_COEFF_FQ2_C1, FROBENIUS_COEFF_FQ6_C1, FROBENIUS_COEFF_FQ6_C2,
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
        let t1 = (self.0[2] * c1).mul_by_nonres() + a_a;
        let t2 = (c0 + c1) * (self.0[0] + self.0[1]) - a_a - b_b;
        let t3 = self.0[2] * c0 + b_b;

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

    fn mul_by_014(self, c0: Fq2, c1: Fq2, c4: Fq2) -> Self {
        let aa = self.0[0].mul_by_01(c0, c1);
        let bb = self.0[1].mul_by_1(c4);
        let o = c1 + c4;
        let c1 = self.0[1] + self.0[0];
        let c1 = c1.mul_by_01(c0, o);
        let c0 = bb;
        let c0 = c0.mul_by_nonres();

        Self([c0 + aa, c1 - aa - bb])
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
        let b = Fq2([Fq::one(); 2]);
        for _ in 0..1000 {
            let a = Fq2::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonres(), expected)
        }
    }

    #[test]
    fn fq6_mul_nonresidue_test() {
        let b = Fq6([Fq2::zero(), Fq2::one(), Fq2::zero()]);
        for _ in 0..1000 {
            let a = Fq6::random(OsRng);
            let expected = a * b;

            assert_eq!(a.mul_by_nonres(), expected)
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
