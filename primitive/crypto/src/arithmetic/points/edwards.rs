use crate::common::Extended;

/// twisted edwards coordinate addition
pub fn add_point<P: Extended>(lhs: P, rhs: P) -> P {
    let a = lhs.get_x() + rhs.get_x();
    let b = lhs.get_y() + rhs.get_y();
    let c = P::PARAM_D * lhs.get_t() * rhs.get_t();
    let d = lhs.get_z() + rhs.get_z();
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
