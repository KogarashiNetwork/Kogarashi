use crate::params::{GENERATOR_X, GENERATOR_Y, PARAM_B};
use bn_254::{Fq, Fr};
use core::borrow::Borrow;
use core::iter::Sum;
use zkstd::arithmetic::weierstrass::*;
use zkstd::common::*;
use zkstd::macros::curve::weierstrass::*;

const B3: Fr = PARAM_B.add_const(PARAM_B).add_const(PARAM_B);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Affine {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    is_infinity: bool,
}

impl Affine {
    pub const RAW_SIZE: usize = 97;

    pub fn from_slice_unchecked(bytes: &[u8]) -> Self {
        let mut x = [0u64; 4];
        let mut y = [0u64; 4];
        let mut z = [0u8; 8];

        bytes
            .chunks_exact(8)
            .zip(x.iter_mut().chain(y.iter_mut()))
            .for_each(|(c, n)| {
                z.copy_from_slice(c);
                *n = u64::from_le_bytes(z);
            });

        let x = Fr::new_unchecked(x);
        let y = Fr::new_unchecked(y);

        let is_infinity = if bytes.len() >= Self::RAW_SIZE {
            bytes[Self::RAW_SIZE - 1] == 1
        } else {
            false
        };

        Self { x, y, is_infinity }
    }

    pub fn to_raw_bytes(&self) -> [u8; Self::RAW_SIZE] {
        let mut bytes = [0u8; Self::RAW_SIZE];
        let chunks = bytes.chunks_mut(8);

        self.x
            .inner()
            .iter()
            .chain(self.y.inner().iter())
            .zip(chunks)
            .for_each(|(n, c)| c.copy_from_slice(&n.to_le_bytes()));

        bytes[Self::RAW_SIZE - 1] = self.is_infinity.into();

        bytes
    }
}

impl Add for Affine {
    type Output = Projective;

    fn add(self, rhs: Affine) -> Self::Output {
        add_affine_point(self, rhs)
    }
}

impl Neg for Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            is_infinity: self.is_infinity,
        }
    }
}

impl Sub for Affine {
    type Output = Projective;

    fn sub(self, rhs: Affine) -> Self::Output {
        add_affine_point(self, rhs.neg())
    }
}

impl Mul<Fq> for Affine {
    type Output = Projective;

    fn mul(self, rhs: Fq) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<Affine> for Fq {
    type Output = Projective;

    fn mul(self, rhs: Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Projective {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    pub(crate) z: Fr,
}

impl Projective {
    /// Converts a batch of `G1Projective` elements into `G1Affine` elements. This
    /// function will panic if `p.len() != q.len()`.
    pub fn batch_normalize(p: &[Self], q: &mut [Affine]) {
        assert_eq!(p.len(), q.len());

        p.iter()
            .zip(q.iter_mut())
            .for_each(|(a, b)| *b = Affine::from(*a))
    }
}

impl Add for Projective {
    type Output = Self;

    fn add(self, rhs: Projective) -> Self {
        add_projective_point(self, rhs)
    }
}

impl Neg for Projective {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Sub for Projective {
    type Output = Self;

    fn sub(self, rhs: Projective) -> Self {
        add_projective_point(self, -rhs)
    }
}

impl Mul<Fq> for Projective {
    type Output = Projective;

    fn mul(self, rhs: Fq) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<Projective> for Fq {
    type Output = Projective;

    fn mul(self, rhs: Projective) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

impl<T> Sum<T> for Projective
where
    T: Borrow<Projective>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::ADDITIVE_IDENTITY, |acc, item| acc + *item.borrow())
    }
}

weierstrass_curve_operation!(
    Fq,
    Fr,
    PARAM_B,
    B3,
    Affine,
    Projective,
    GENERATOR_X,
    GENERATOR_Y
);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use rand_core::OsRng;

    curve_test!(grumpkin, Fq, Affine, Projective, 100);

    #[test]
    fn test_batch_normalize() {
        let a = Projective::ADDITIVE_GENERATOR.double();
        let b = a.double();
        let c = b.double();

        for a_identity in (0..1).map(|n| n == 1) {
            for b_identity in (0..1).map(|n| n == 1) {
                for c_identity in (0..1).map(|n| n == 1) {
                    let mut v = [a, b, c];
                    if a_identity {
                        v[0] = Projective::ADDITIVE_IDENTITY
                    }
                    if b_identity {
                        v[1] = Projective::ADDITIVE_IDENTITY
                    }
                    if c_identity {
                        v[2] = Projective::ADDITIVE_IDENTITY
                    }

                    let mut t = [
                        Affine::ADDITIVE_IDENTITY,
                        Affine::ADDITIVE_IDENTITY,
                        Affine::ADDITIVE_IDENTITY,
                    ];
                    let expected = [Affine::from(v[0]), Affine::from(v[1]), Affine::from(v[2])];

                    Projective::batch_normalize(&v[..], &mut t[..]);

                    assert_eq!(&t[..], &expected[..]);
                }
            }
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn grumpkin_operations() {
        let aff1 = Affine::random(OsRng);
        let aff2 = Affine::random(OsRng);
        let mut ext1 = Projective::random(OsRng);
        let ext2 = Projective::random(OsRng);
        let scalar = Fq::from(42);

        let _ = aff1 + aff2;
        let _ = &aff1 + &aff2;
        let _ = &aff1 + aff2;
        let _ = aff1 + &aff2;

        let _ = aff1 + ext1;
        let _ = &aff1 + &ext1;
        let _ = &aff1 + ext1;
        let _ = aff1 + &ext1;
        let _ = ext1 + aff1;
        let _ = &ext1 + &aff1;
        let _ = &ext1 + aff1;
        let _ = ext1 + &aff1;

        let _ = ext1 + ext2;
        let _ = &ext1 + &ext2;
        let _ = &ext1 + ext2;
        let _ = ext1 + &ext2;
        ext1 += ext2;
        ext1 += &ext2;
        ext1 += aff2;
        ext1 += &aff2;

        let _ = aff1 - aff2;
        let _ = &aff1 - &aff2;
        let _ = &aff1 - aff2;
        let _ = aff1 - &aff2;

        let _ = aff1 - ext1;
        let _ = &aff1 - &ext1;
        let _ = &aff1 - ext1;
        let _ = aff1 - &ext1;
        let _ = ext1 - aff1;
        let _ = &ext1 - &aff1;
        let _ = &ext1 - aff1;
        let _ = ext1 - &aff1;

        let _ = ext1 - ext2;
        let _ = &ext1 - &ext2;
        let _ = &ext1 - ext2;
        let _ = ext1 - &ext2;
        ext1 -= ext2;
        ext1 -= &ext2;
        ext1 -= aff2;
        ext1 -= &aff2;

        let _ = aff1 * scalar;
        let _ = aff1 * &scalar;
        let _ = &aff1 * scalar;
        let _ = &aff1 * &scalar;
        let _ = scalar * aff1;
        let _ = &scalar * &aff1;
        let _ = scalar * &aff1;
        let _ = &scalar * aff1;

        let _ = ext1 * scalar;
        let _ = ext1 * &scalar;
        let _ = &ext1 * scalar;
        let _ = &ext1 * &scalar;
        let _ = scalar * ext1;
        let _ = &scalar * &ext1;
        let _ = scalar * &ext1;
        let _ = &scalar * ext1;
        ext1 *= scalar;
        ext1 *= &scalar;
    }
}
