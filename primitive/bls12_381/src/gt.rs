use crate::{Fq12, Fr};
use zkstd::common::*;

/// pairing target group of 12 degree extension of field
#[derive(Debug, Clone, Copy)]
pub struct Gt(pub Fq12);

impl Group for Gt {
    type Scalar = Fr;

    const ADDITIVE_GENERATOR: Self = Self(Fq12::generator());
    const ADDITIVE_IDENTITY: Self = Self(Fq12::one());

    fn zero() -> Self {
        Self(Fq12::zero())
    }

    fn invert(self) -> Option<Self> {
        unimplemented!()
    }

    fn random(rand: impl RngCore) -> Self {
        Self(Fq12::random(rand))
    }
}

impl Eq for Gt {}

impl PartialEq for Gt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add for Gt {
    type Output = Gt;

    fn add(self, rhs: Gt) -> Gt {
        Gt(self.0 * rhs.0)
    }
}

impl Neg for Gt {
    type Output = Gt;

    fn neg(self) -> Gt {
        Gt(self.0.conjugate())
    }
}

impl Sub for Gt {
    type Output = Gt;

    fn sub(self, rhs: Gt) -> Gt {
        self + (-rhs)
    }
}

impl Mul<Fr> for Gt {
    type Output = Gt;

    fn mul(self, other: Fr) -> Self::Output {
        let mut res = Self::Output::ADDITIVE_IDENTITY;
        let mut acc = self;
        let bits: Vec<u8> = other
            .to_bits()
            .into_iter()
            .skip_while(|x| *x == 0)
            .collect();
        for &bit in bits.iter().rev() {
            if bit == 1 {
                res += acc;
            }
            acc = acc.double();
        }

        acc
    }
}

impl MulAssign<Fr> for Gt {
    fn mul_assign(&mut self, rhs: Fr) {
        *self = *self * rhs;
    }
}

impl AddAssign for Gt {
    fn add_assign(&mut self, rhs: Gt) {
        *self = *self + rhs;
    }
}

impl SubAssign for Gt {
    fn sub_assign(&mut self, rhs: Gt) {
        *self = *self - rhs;
    }
}

impl Gt {
    pub fn double(&self) -> Gt {
        Gt(self.0.square())
    }
}
