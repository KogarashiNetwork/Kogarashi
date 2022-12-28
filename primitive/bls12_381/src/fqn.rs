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
        let mut aa = self.0[0];
        aa *= rhs.0[0];
        let mut bb = self.0[1];
        bb *= rhs.0[1];
        let mut o = rhs.0[0];
        o += rhs.0[1];
        let mut tmp = self.0[0] + self.0[1];
        tmp *= o;
        tmp -= aa;
        tmp -= bb;
        let mut tmp2 = bb;
        tmp2 = tmp2.mul_by_nonresidue();
        tmp2 += aa;
        Self([tmp, tmp2])
    }

    fn mul_by_nonres(self) -> Self {
        unimplemented!()
    }
}
