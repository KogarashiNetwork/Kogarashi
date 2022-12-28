use crate::fq::Fq;
use crate::g1::G1Affine;
use crate::g2::PairingCoeff;
use zero_crypto::dress::extension_field::*;
use zero_crypto::dress::pairing::bls12_range_field_pairing;

// sextic twist of Fp12
// degree 2 extension field
const TWO_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq2, Fq, TWO_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 6 extension field
const SIX_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 3;
extension_field_operation!(Fq6, Fq2, SIX_DEGREE_EXTENTION_LIMBS_LENGTH);

// degree 12 extension field
const TWELV_DEGREE_EXTENTION_LIMBS_LENGTH: usize = 2;
extension_field_operation!(Fq12, Fq6, TWELV_DEGREE_EXTENTION_LIMBS_LENGTH);

// pairing extension for degree 12 extension field
bls12_range_field_pairing!(Fq12, Fq2, G1Affine, PairingCoeff);

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
        self * self
    }

    fn mul_by_nonres(self) -> Self {
        Self([self.0[0] - self.0[1], self.0[0] + self.0[1]])
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
}
