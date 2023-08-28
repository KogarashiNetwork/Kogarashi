use crate::arithmetic::utils::Naf;
use crate::common::{CurveGroup, PrimeField, Ring, TwistedEdwardsAffine, TwistedEdwardsExtended};

/// twisted edwards coordinate addition
/// 9M + 4A + 3B
#[inline(always)]
pub fn add_affine_point<P: TwistedEdwardsAffine>(lhs: P, rhs: P) -> P::Extended {
    let (x0, y0) = (lhs.get_x(), lhs.get_y());
    let (x1, y1) = (rhs.get_x(), rhs.get_y());

    let a = x0 * x1;
    let b = y0 * y1;
    let c = P::PARAM_D * a * b;
    let e = (x0 + y0) * (x1 + y1) - a - b;
    let f = P::Range::one() - c;
    let g = P::Range::one() + c;
    let h = b + a;

    let x = e * f;
    let y = g * h;
    let t = e * h;
    let z = f * g;

    P::new_extended(x, y, t, z)
}

/// twisted edwards extended coordinate doubling
/// 4M + 3S + 1D + 3B + 2A
#[inline(always)]
pub fn double_affine_point<P: TwistedEdwardsAffine>(lhs: P) -> P::Extended {
    let (x, y) = (lhs.get_x(), lhs.get_y());

    let a = x.square();
    let b = y.square();
    let c = P::Range::one().double();
    let d = -a - b;
    let e = (x + y).square() + d;
    let g = b - a;
    let f = g - c;

    let x = e * f;
    let y = g * d;
    let t = e * d;
    let z = f * g;

    P::new_extended(x, y, t, z)
}

/// twisted edwards extended coordinate addition
/// 10M + 4A + 3B
#[inline(always)]
pub fn add_projective_point<P: TwistedEdwardsExtended>(lhs: P, rhs: P) -> P {
    let (x0, y0, z0, t0) = (lhs.get_x(), lhs.get_y(), lhs.get_z(), lhs.get_t());
    let (x1, y1, z1, t1) = (rhs.get_x(), rhs.get_y(), rhs.get_z(), rhs.get_t());

    let a = x0 * x1;
    let b = y0 * y1;
    let c = P::PARAM_D * t0 * t1;
    let d = z0 * z1;
    let e = (x0 + y0) * (x1 + y1) - a - b;
    let f = d - c;
    let g = d + c;
    let h = b + a;

    let x = e * f;
    let y = g * h;
    let t = e * h;
    let z = f * g;

    P::new(x, y, t, z)
}

/// twisted edwards extended coordinate doubling
/// 5M + 3S + 2D + 2B + 2A
#[inline(always)]
pub fn double_projective_point<P: TwistedEdwardsExtended>(lhs: P) -> P {
    let (x, y, z) = (lhs.get_x(), lhs.get_y(), lhs.get_z());

    let a = -x.square();
    let b = y.square();
    let c = z.square().double();
    let d = a - b;
    let e = (x * y).double();
    let g = a + b;
    let f = g - c;

    let x = e * f;
    let y = g * d;
    let t = e * d;
    let z = f * g;

    P::new(x, y, t, z)
}

/// coordinate scalar
#[inline(always)]
pub fn scalar_point<P: TwistedEdwardsExtended>(point: P, scalar: &<P as CurveGroup>::Scalar) -> P {
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
