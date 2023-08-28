use crate::arithmetic::utils::Naf;
use crate::common::{CurveGroup, PrimeField, Projective, WeierstrassCurve};

/// weierstrass projective coordinate addition
#[inline(always)]
pub fn add_point<P: Projective>(lhs: P, rhs: P) -> P {
    let (x0, y0, z0) = (lhs.get_x(), lhs.get_y(), lhs.get_z());
    let (x1, y1, z1) = (rhs.get_x(), rhs.get_y(), rhs.get_z());

    if lhs.is_identity() {
        return rhs;
    } else if rhs.is_identity() {
        return lhs;
    }

    let s1 = y0 * z1;
    let s2 = y1 * z0;
    let u1 = x0 * z1;
    let u2 = x1 * z0;

    if u1 == u2 {
        if s1 == s2 {
            return double_point(lhs);
        } else {
            return <P as CurveGroup>::ADDITIVE_IDENTITY;
        }
    }

    let s = s1 - s2;
    let u = u1 - u2;
    let uu = u.square();
    let v = z0 * z1;
    let w = s.square() * v - uu * (u1 + u2);
    let uuu = uu * u;

    let x = u * w;
    let y = s * (u1 * uu - w) - s1 * uuu;
    let z = uuu * v;

    P::new(x, y, z)
}

/// weierstrass projective coordinate doubling
#[inline(always)]
pub fn double_point<P: Projective>(point: P) -> P {
    // Algorithm 9, https://eprint.iacr.org/2015/1060.pdf
    if point.is_identity() {
        <P as CurveGroup>::ADDITIVE_IDENTITY
    } else {
        let b3 = <P as WeierstrassCurve>::PARAM_3B;
        let (x, y, z) = (point.get_x(), point.get_y(), point.get_z());

        let t0 = y.square();
        let z3 = t0.double().double().double();
        let t1 = y * z;
        let t2 = z.square();
        let t2 = b3 * t2;
        let x3 = t2 * z3;
        let y3 = t0 + t2;
        let z3 = t1 * z3;
        let t1 = t2.double();
        let t2 = t1 + t2;
        let t0 = t0 - t2;
        let y3 = t0 * y3;
        let y3 = x3 + y3;
        let t1 = x * y;
        let x3 = t0 * t1;
        let x3 = x3.double();

        P::new(x3, y3, z3)
    }
}

/// coordinate scalar
#[inline(always)]
pub fn scalar_point<P: Projective>(point: P, scalar: &<P as CurveGroup>::Scalar) -> P {
    let mut res = P::ADDITIVE_IDENTITY;
    let mut acc = point;
    for &naf in scalar.to_nafs().iter() {
        if naf == Naf::Plus {
            res += acc;
        } else if naf == Naf::Minus {
            res -= acc;
        }
        acc = Into::<P>::into(acc.double());
    }
    res
}
