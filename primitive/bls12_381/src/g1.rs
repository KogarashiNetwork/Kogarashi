use crate::params::{
    BLS_X, BLS_X_IS_NEGATIVE, G1_GENERATOR_X, G1_GENERATOR_Y, G1_PARAM_A, G1_PARAM_B,
};
use crate::{Fq, Fr};
use core::borrow::Borrow;
use core::iter::Sum;
use dusk_bytes::{Error as BytesError, Serializable};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zero_crypto::arithmetic::weierstrass::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::weierstrass::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Projective {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    pub(crate) z: Fq,
}

impl Add for G1Projective {
    type Output = Self;

    fn add(self, rhs: G1Projective) -> Self {
        add_point(self, rhs)
    }
}

impl Neg for G1Projective {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Sub for G1Projective {
    type Output = Self;

    fn sub(self, rhs: G1Projective) -> Self {
        add_point(self, -rhs)
    }
}

impl Mul<Fr> for G1Projective {
    type Output = G1Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<G1Projective> for Fr {
    type Output = G1Projective;

    fn mul(self, rhs: G1Projective) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Affine {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    is_infinity: bool,
}

impl Add for G1Affine {
    type Output = G1Projective;

    fn add(self, rhs: G1Affine) -> Self::Output {
        add_point(self.to_extended(), rhs.to_extended())
    }
}

impl Neg for G1Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            is_infinity: self.is_infinity,
        }
    }
}

impl Sub for G1Affine {
    type Output = G1Projective;

    fn sub(self, rhs: G1Affine) -> Self::Output {
        add_point(self.to_extended(), rhs.neg().to_extended())
    }
}

impl Mul<Fr> for G1Affine {
    type Output = G1Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<G1Affine> for Fr {
    type Output = G1Projective;

    fn mul(self, rhs: G1Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

weierstrass_curve_operation!(
    Fr,
    Fq,
    G1_PARAM_A,
    G1_PARAM_B,
    G1Affine,
    G1Projective,
    G1_GENERATOR_X,
    G1_GENERATOR_Y
);

// below here, the crate uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and
// [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by
// Dusk-Network team and, @str4d and @ebfull

pub const BETA: Fq = Fq([
    0x30f1361b798a64e8,
    0xf3b8ddab7ece5a2a,
    0x16a8ca3ac61577f7,
    0xc26a2ff874fd029b,
    0x3636b76660701c6e,
    0x051ba4ab241b6160,
]);

const B: Fq = Fq([
    0xaa270000000cfff3,
    0x53cc0032fc34000a,
    0x478fe97a6b0a807f,
    0xb1d37ebee6ba24d7,
    0x8ec9733bbf78ab2f,
    0x9d645513d83de7e,
]);

fn endomorphism(p: &G1Affine) -> G1Affine {
    // Endomorphism of the points on the curve.
    // endomorphism_p(x,y) = (BETA * x, y)
    // where BETA is a non-trivial cubic root of unity in Fq.
    let mut res = *p;
    res.x *= BETA;
    res
}

impl G1Affine {
    pub const RAW_SIZE: usize = 97;

    pub fn from_slice_unchecked(bytes: &[u8]) -> Self {
        let mut x = [0u64; 6];
        let mut y = [0u64; 6];
        let mut z = [0u8; 8];

        bytes
            .chunks_exact(8)
            .zip(x.iter_mut().chain(y.iter_mut()))
            .for_each(|(c, n)| {
                z.copy_from_slice(c);
                *n = u64::from_le_bytes(z);
            });

        let x = Fq(x);
        let y = Fq(y);

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
            .internal_repr()
            .iter()
            .chain(self.y.internal_repr().iter())
            .zip(chunks)
            .for_each(|(n, c)| c.copy_from_slice(&n.to_le_bytes()));

        bytes[Self::RAW_SIZE - 1] = self.is_infinity.into();

        bytes
    }

    pub fn is_torsion_free(&self) -> Choice {
        // Algorithm from Section 6 of https://eprint.iacr.org/2021/1130
        // Updated proof of correctness in https://eprint.iacr.org/2022/352
        //
        // Check that endomorphism_p(P) == -[x^2] P

        let minus_x_squared_times_p = G1Projective::from(*self).mul_by_x().mul_by_x().neg();
        let endomorphism_p = endomorphism(self);
        minus_x_squared_times_p.ct_eq(&G1Projective::from(endomorphism_p))
    }
}

impl G1Projective {
    /// Multiply `self` by `crate::BLS_X`, using double and add.
    fn mul_by_x(&self) -> G1Projective {
        let mut xself = G1Projective::ADDITIVE_IDENTITY;
        // NOTE: in BLS12-381 we can just skip the first bit.
        let mut x = BLS_X >> 1;
        let mut tmp = *self;
        while x != 0 {
            tmp = tmp.double();

            if x % 2 == 1 {
                xself += tmp;
            }
            x >>= 1;
        }
        // finally, flip the sign
        if BLS_X_IS_NEGATIVE {
            xself = -xself;
        }
        xself
    }

    /// Converts a batch of `G1Projective` elements into `G1Affine` elements. This
    /// function will panic if `p.len() != q.len()`.
    pub fn batch_normalize(p: &[Self], q: &mut [G1Affine]) {
        assert_eq!(p.len(), q.len());

        p.iter()
            .zip(q.iter_mut())
            .for_each(|(a, b)| *b = G1Affine::from(*a))
    }
}

impl Serializable<48> for G1Affine {
    type Error = BytesError;

    /// Serializes this element into compressed form. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        // Strictly speaking, self.x is zero already when self.infinity is true, but
        // to guard against implementation mistakes we do not assume this.
        let mut res = Fq::conditional_select(&self.x, &Fq::zero(), (self.is_infinity as u8).into())
            .to_bytes();

        // This point is in compressed form, so we set the most significant bit.
        res[0] |= 1u8 << 7;

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= u8::conditional_select(&0u8, &(1u8 << 6), (self.is_infinity as u8).into());

        // Is the y-coordinate the lexicographically largest of the two associated with the
        // x-coordinate? If so, set the third-most significant bit so long as this is not
        // the point at infinity.
        res[0] |= u8::conditional_select(
            &0u8,
            &(1u8 << 5),
            (!Choice::from(self.is_infinity as u8)) & self.y.lexicographically_largest(),
        );

        res
    }

    /// Attempts to deserialize a compressed element. See [`notes::serialization`](crate::notes::serialization)
    /// for details about how group elements are serialized.
    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        // We already know the point is on the curve because this is established
        // by the y-coordinate recovery procedure in from_compressed_unchecked().

        let compression_flag_set = Choice::from((buf[0] >> 7) & 1);
        let infinity_flag_set = Choice::from((buf[0] >> 6) & 1);
        let sort_flag_set = Choice::from((buf[0] >> 5) & 1);

        // Attempt to obtain the x-coordinate
        let x = {
            let mut tmp = [0; Self::SIZE];
            tmp.copy_from_slice(&buf[..Self::SIZE]);

            // Mask away the flag bits
            tmp[0] &= 0b0001_1111;

            Fq::from_bytes(&tmp)
        };

        let x: Option<Self> = x
            .and_then(|x| {
                // If the infinity flag is set, return the value assuming
                // the x-coordinate is zero and the sort bit is not set.
                //
                // Otherwise, return a recovered point (assuming the correct
                // y-coordinate can be found) so long as the infinity flag
                // was not set.

                CtOption::new(
                    G1Affine::ADDITIVE_IDENTITY,
                    infinity_flag_set & // Infinity flag should be set
                compression_flag_set & // Compression flag should be set
                (!sort_flag_set) & // Sort flag should not be set
                (x.is_zero() as u8).into(), // The x-coordinate should be zero
                )
                .or_else(|| {
                    // Recover a y-coordinate given x by y = sqrt(x^3 + 4)
                    ((x.square() * x) + B).sqrt().and_then(|y| {
                        // Switch to the correct y-coordinate if necessary.
                        let y = Fq::conditional_select(
                            &y,
                            &-y,
                            y.lexicographically_largest() ^ sort_flag_set,
                        );

                        CtOption::new(
                            G1Affine {
                                x,
                                y,
                                is_infinity: infinity_flag_set.into(),
                            },
                            (!infinity_flag_set) & // Infinity flag should not be set
                        compression_flag_set, // Compression flag should be set
                        )
                    })
                })
            })
            .and_then(|p| CtOption::new(p, p.is_torsion_free()))
            .into();

        x.ok_or(BytesError::InvalidData)
    }
}

impl<T> Sum<T> for G1Projective
where
    T: Borrow<G1Projective>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::ADDITIVE_IDENTITY, |acc, item| acc + *item.borrow())
    }
}

impl ConditionallySelectable for G1Affine {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G1Affine {
            x: Fq::conditional_select(&a.x, &b.x, choice),
            y: Fq::conditional_select(&a.y, &b.y, choice),
            is_infinity: ConditionallySelectable::conditional_select(
                &Choice::from(a.is_infinity as u8),
                &Choice::from(b.is_infinity as u8),
                choice,
            )
            .into(),
        }
    }
}

impl ConstantTimeEq for G1Projective {
    fn ct_eq(&self, other: &Self) -> Choice {
        // Is (xz^2, yz^3, z) equal to (x'z'^2, yz'^3, z') when converted to affine?

        let self_is_zero = Choice::from(self.is_identity() as u8);
        let other_is_zero = Choice::from(other.is_identity() as u8);

        let is_same = self.x * other.z == other.x * self.z && self.y * other.z == other.y * self.z;

        (self_is_zero & other_is_zero) // Both point at infinity
            | Choice::from(is_same as u8)
        // Neither point at infinity, coordinates are the same
    }
}

impl ConditionallySelectable for G1Projective {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        G1Projective {
            x: Fq::conditional_select(&a.x, &b.x, choice),
            y: Fq::conditional_select(&a.y, &b.y, choice),
            z: Fq::conditional_select(&a.z, &b.z, choice),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(bls12_381, Fr, G1Affine, G1Projective, 100);

    #[test]
    fn test_batch_normalize() {
        let a = G1Projective::ADDITIVE_GENERATOR.double();
        let b = a.double();
        let c = b.double();

        for a_identity in (0..1).map(|n| n == 1) {
            for b_identity in (0..1).map(|n| n == 1) {
                for c_identity in (0..1).map(|n| n == 1) {
                    let mut v = [a, b, c];
                    if a_identity {
                        v[0] = G1Projective::ADDITIVE_IDENTITY
                    }
                    if b_identity {
                        v[1] = G1Projective::ADDITIVE_IDENTITY
                    }
                    if c_identity {
                        v[2] = G1Projective::ADDITIVE_IDENTITY
                    }

                    let mut t = [
                        G1Affine::ADDITIVE_IDENTITY,
                        G1Affine::ADDITIVE_IDENTITY,
                        G1Affine::ADDITIVE_IDENTITY,
                    ];
                    let expected = [
                        G1Affine::from(v[0]),
                        G1Affine::from(v[1]),
                        G1Affine::from(v[2]),
                    ];

                    G1Projective::batch_normalize(&v[..], &mut t[..]);

                    assert_eq!(&t[..], &expected[..]);
                }
            }
        }
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn bls_operations() {
        let aff1 = G1Affine::random(OsRng).to_affine();
        let aff2 = G1Affine::random(OsRng).to_affine();
        let mut ext1 = G1Projective::random(OsRng);
        let ext2 = G1Projective::random(OsRng);
        let scalar = Fr::from(42);

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
