use crate::arithmetic::utils::Naf;
use crate::common::{CurveGroup, PrimeField, TwistedEdwardsExtended};

/// twisted edwards coordinate addition
/// 10M + 4A + 3B
#[inline(always)]
pub fn add_point<P: TwistedEdwardsExtended>(lhs: P, rhs: P) -> P {
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

/// twisted edwards coordinate doubling
/// 4M + 4S + 1D + 4B + 1A
#[inline(always)]
pub fn double_point<P: TwistedEdwardsExtended>(lhs: P) -> P {
    let (x, y, z) = (lhs.get_x(), lhs.get_y(), lhs.get_z());

    let a = x.square();
    let b = y.square();
    let c = z.square().double();
    let d = -a;
    let e = (x + y).square() - a - b;
    let g = d + b;
    let f = g - c;
    let h = d - b;

    let x = e * f;
    let y = g * h;
    let t = e * h;
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
