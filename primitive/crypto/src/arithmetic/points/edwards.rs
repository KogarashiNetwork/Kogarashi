use crate::arithmetic::utils::Naf;
use crate::common::{CurveGroup, PrimeField, TwistedEdwardsExtended};

/// twisted edwards coordinate addition
/// 10M + 4A + 3B
pub fn add_point<P: TwistedEdwardsExtended>(lhs: P, rhs: P) -> P {
    let a = lhs.get_x() * rhs.get_x();
    let b = lhs.get_y() * rhs.get_y();
    let c = P::PARAM_D * lhs.get_t() * rhs.get_t();
    let d = lhs.get_z() * rhs.get_z();
    let e = (lhs.get_x() + lhs.get_y()) * (rhs.get_x() + rhs.get_y()) - a - b;
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
pub fn double_point<P: TwistedEdwardsExtended>(lhs: P) -> P {
    let a = lhs.get_x().square();
    let b = lhs.get_y().square();
    let c = lhs.get_z().square().double();
    let d = -a;
    let e = (lhs.get_x() + lhs.get_y()).square() - a - b;
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
