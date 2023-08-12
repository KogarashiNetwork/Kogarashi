use bls_12_381::{Fr, G1Affine, G1Projective};
use ec_pairing::{msm_curve_addtion, TatePairing};
use rand_core::OsRng;
use zkstd::behave::{Group, Projective};
use zkstd::common::{Affine, CurveGroup};

fn customized_scalar_point<P: Projective<Extended = P>>(point: P, scalar: &Fr) -> P {
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
