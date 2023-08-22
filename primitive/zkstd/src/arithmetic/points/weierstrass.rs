use crate::arithmetic::utils::Naf;
use crate::common::{CurveGroup, PrimeField, Projective, WeierstrassCurve};

/// weierstrass projective coordinate addition
#[inline(always)]
pub fn add_point<P: Projective>(lhs: P, rhs: P) -> P {
    let b3 = <P as WeierstrassCurve>::PARAM_3B;
    let (x0, y0, z0) = (lhs.get_x(), lhs.get_y(), lhs.get_z());
    let (x1, y1, z1) = (rhs.get_x(), rhs.get_y(), rhs.get_z());

    let t0 = x0 * x1;
    let t1 = y0 * y1;
    let t2 = z0 * z1;
    let t3 = x0 + y0;
    let t4 = x1 + y1;
    let t3 = t3 * t4;
    let t4 = t0 + t1;
    let t3 = t3 - t4;
    let t4 = y0 + z0;
    let x3 = y1 + z1;
    let t4 = t4 * x3;
    let x3 = t1 + t2;
    let t4 = t4 - x3;
    let x3 = x0 + z0;
    let y3 = x1 + z1;
    let x3 = x3 * y3;
    let y3 = t0 + t2;
    let y3 = x3 - y3;
    let x3 = t0.double();
    let t0 = x3 + t0;
    let t2 = b3 * t2;
    let z3 = t1 + t2;
    let t1 = t1 - t2;
    let y3 = b3 * y3;
    let x3 = t4 * y3;
    let t2 = t3 * t1;
    let x3 = t2 - x3;
    let y3 = y3 * t0;
    let t1 = t1 * z3;
    let y3 = t1 + y3;
    let t0 = t0 * t3;
    let z3 = z3 * t4;
    let z3 = z3 + t0;

    P::new(x3, y3, z3)
}

/// weierstrass projective coordinate doubling
#[inline(always)]
pub fn double_point<P: Projective>(point: P) -> P {
    if point.is_identity() || point.get_y().is_zero() {
        <P as CurveGroup>::ADDITIVE_IDENTITY
    } else {
        let xx = point.get_x().square();
        let t = xx.double() + xx;
        let u = (point.get_y() * point.get_z()).double();
        let v = (u * point.get_x() * point.get_y()).double();
        let w = t.square() - v.double();
        let uu = u.square();

        let x = u * w;
        let y = t * (v - w) - (uu * point.get_y().square()).double();
        let z = uu * u;

        P::new(x, y, z)
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
