use crate::common::{Group, PrimeField, Projective};

/// The projective coordinate addition
pub fn add_point<P: Projective>(lhs: P, rhs: P) -> P {
    if lhs.is_identity() {
        return rhs;
    } else if rhs.is_identity() {
        return lhs;
    }

    let s1 = lhs.get_y() * rhs.get_z();
    let s2 = rhs.get_y() * lhs.get_z();
    let u1 = lhs.get_x() * rhs.get_z();
    let u2 = rhs.get_x() * lhs.get_z();

    if u1 == u2 {
        if s1 == s2 {
            return double_point(lhs);
        } else {
            return <P as Group>::ADDITIVE_IDENTITY;
        }
    }

    let s = s1 - s2;
    let u = u1 - u2;
    let uu = u.square();
    let v = lhs.get_z() * rhs.get_z();
    let w = s.square() * v - uu * (u1 + u2);
    let uuu = uu * u;

    let mut res = <P as Default>::default();
    res.set_x(u * w);
    res.set_y(s * (u1 * uu - w) - s1 * uuu);
    res.set_z(uuu * v);
    res
}

/// The projective coordinate doubling

pub fn double_point<P: Projective>(point: P) -> P {
    if point.is_identity() || point.get_y().is_zero() {
        <P as Group>::ADDITIVE_IDENTITY
    } else {
        let xx = point.get_x().square();
        let t = xx.double() + xx;
        let u = (point.get_y() * point.get_z()).double();
        let v = (u * point.get_x() * point.get_y()).double();
        let w = t.square() - v.double();
        let uu = u.square();
        let mut res = <P as Default>::default();
        res.set_x(u * w);
        res.set_y(t * (v - w) - (uu * point.get_y().square()).double());
        res.set_z(uu * u);
        res
    }
}
