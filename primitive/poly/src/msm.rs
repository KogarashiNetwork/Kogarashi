#[cfg(feature = "std")]
use rayon::prelude::*;
use zkstd::common::{vec, Curve, CurveGroup, Pairing, SigUtils, Vec};

/// Performs a Variable Base Multiscalar Multiplication.
pub fn msm_curve_addtion<P: Pairing>(
    bases: &[P::G1Affine],
    coeffs: &[P::ScalarField],
) -> P::G1Projective {
    let c = if bases.len() < 4 {
        1
    } else if bases.len() < 32 {
        3
    } else {
        let log2 = usize::BITS - bases.len().leading_zeros();
        (log2 * 69 / 100) as usize + 2
    };

    let mut buckets: Vec<Vec<Bucket<P>>> = vec![vec![Bucket::None; (1 << c) - 1]; (256 / c) + 1];
    #[cfg(feature = "std")]
    let bucket_iteration = buckets.par_iter_mut();
    #[cfg(not(feature = "std"))]
    let bucket_iteration = buckets.iter_mut();
    let filled_buckets = bucket_iteration
        .enumerate()
        .rev()
        .map(|(i, bucket)| {
            for (coeff, base) in coeffs.iter().zip(bases.iter()) {
                let seg = get_at(i, c, coeff.to_bytes());
                if seg != 0 {
                    bucket[seg - 1].add_assign(base);
                }
            }
            // Summation by parts
            // e.g. 3a + 2b + 1c = a +
            //                    (a) + b +
            //                    ((a) + b) + c
            let mut acc = P::G1Projective::ADDITIVE_IDENTITY;
            let mut sum = P::G1Projective::ADDITIVE_IDENTITY;
            bucket.iter().rev().for_each(|b| {
                sum = b.add(sum);
                acc += sum;
            });
            (0..c * i).for_each(|_| acc = acc.double());
            acc
        })
        .collect::<Vec<_>>();
    filled_buckets
        .iter()
        .fold(P::G1Projective::ADDITIVE_IDENTITY, |a, b| a + b)
}

#[derive(Clone, Copy)]
enum Bucket<P: Pairing> {
    None,
    Affine(P::G1Affine),
    Projective(P::G1Projective),
}

impl<P: Pairing> Bucket<P> {
    fn add_assign(&mut self, other: &P::G1Affine) {
        *self = match *self {
            Bucket::None => Bucket::Affine(*other),
            Bucket::Affine(a) => Bucket::Projective(a + other),
            Bucket::Projective(a) => Bucket::Projective(a + other),
        }
    }

    fn add(&self, other: P::G1Projective) -> P::G1Projective {
        match self {
            Bucket::None => other,
            Bucket::Affine(a) => other + a,
            Bucket::Projective(a) => other + a,
        }
    }
}

fn get_at(segment: usize, c: usize, bytes: [u8; 32]) -> usize {
    let skip_bits = segment * c;
    let skip_bytes = skip_bits / 8;

    if skip_bytes >= 32 {
        0
    } else {
        let mut v = [0; 8];
        for (v, o) in v.iter_mut().zip(bytes[skip_bytes..].iter()) {
            *v = *o;
        }

        let mut tmp = u64::from_le_bytes(v);
        tmp >>= skip_bits - (skip_bytes * 8);
        (tmp % (1 << c)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::msm_curve_addtion;
    use bls_12_381::{Fr, G1Affine, G1Projective};
    use ec_pairing::TatePairing;
    use rand_core::OsRng;
    use zkstd::behave::{Group, WeierstrassProjective};
    use zkstd::common::{Affine, CurveGroup};

    fn customized_scalar_point<P: WeierstrassProjective<Extended = P>>(point: P, scalar: &Fr) -> P {
        let mut res = P::ADDITIVE_IDENTITY;
        let one = point;
        let two = one + point;
        let three = two + point;
        for &bit in scalar.to_costomized_repr().iter() {
            res = res.double().double();
            if bit == 1 {
                res += one;
            } else if bit == 2 {
                res += two;
            } else if bit == 3 {
                res += three;
            }
        }
        res
    }

    #[test]
    fn multi_scalar_multiplication_test() {
        let n = 1 << 5;
        let points = (0..n)
            .map(|_| G1Affine::from(G1Affine::random(OsRng)))
            .collect::<Vec<_>>();
        let scalars = (0..n).map(|_| Fr::random(OsRng)).collect::<Vec<_>>();
        let msm = msm_curve_addtion::<TatePairing>(&points[..], &scalars[..]);
        let naive = points
            .iter()
            .rev()
            .zip(scalars.iter().rev())
            .fold(G1Projective::ADDITIVE_IDENTITY, |acc, (point, coeff)| {
                acc + customized_scalar_point(point.to_extended(), coeff)
            });
        assert_eq!(msm, naive);
    }
}
