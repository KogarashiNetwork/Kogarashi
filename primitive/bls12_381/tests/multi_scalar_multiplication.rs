use rand_core::OsRng;
use zero_bls12_381::{msm_variable_base, Fr, G1Affine, G1Projective};
use zero_crypto::behave::{Group, Projective};
use zero_crypto::common::WeierstrassAffine;

fn customized_scalar_point<P: Projective>(point: P, scalar: &Fr) -> P {
    let mut res = P::ADDITIVE_IDENTITY;
    let mut acc = point;
    for &bit in scalar.to_costomized_repr().iter().rev() {
        if bit == 1 {
            res += acc;
        }
        acc = acc.double();
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
    let msm = msm_variable_base(&points[..], &scalars[..]);
    let naive = points
        .iter()
        .rev()
        .zip(scalars.iter().rev())
        .fold(G1Projective::ADDITIVE_IDENTITY, |acc, (point, coeff)| {
            acc + customized_scalar_point(point.to_projective(), coeff)
        });
    assert_eq!(msm, naive);
}
