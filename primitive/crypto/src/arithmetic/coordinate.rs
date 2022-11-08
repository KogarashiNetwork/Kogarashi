use crate::behave::Curve;
use crate::entity::ProjectiveCoordinate;

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub fn add<E: Curve>(
    lhs: ProjectiveCoordinate<E>,
    rhs: ProjectiveCoordinate<E>,
    identity: ProjectiveCoordinate<E>,
) -> ProjectiveCoordinate<E> {
    if lhs == identity {
        return rhs;
    } else if rhs == identity {
        return lhs;
    }

    let s1 = lhs.y * rhs.z;
    let s2 = rhs.y * lhs.z;
    let u1 = lhs.x * rhs.z;
    let u2 = rhs.x * lhs.z;

    if u1 == u2 {
        if s1 == s2 {
            return double(lhs, identity);
        } else {
            return identity;
        }
    } else {
        let s = s1 - s2;
        let u = u1 - u2;
        let uu = u.square();
        let v = lhs.z * rhs.z;
        let w = s.square() * v - uu * (u1 + u2);
        let uuu = uu * u;
        return ProjectiveCoordinate {
            x: u * w,
            y: s * (u1 * uu - w) - s1 * uuu,
            z: uuu * v,
        };
    }
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0, b = 4
pub(crate) fn double<E: Curve>(
    point: ProjectiveCoordinate<E>,
    identity: ProjectiveCoordinate<E>,
) -> ProjectiveCoordinate<E> {
    if point == identity || point.y.is_zero() {
        identity
    } else {
        let xx = point.x.square();
        let t = xx.double() + xx;
        let u = (point.y * point.z).double();
        let v = (u * point.x * point.y).double();
        let w = t.square() - v.double();
        Coordinate {
            x: u * w,
            y: t * (v - w) - (u.square() * point.y.square()).double(),
            z: u.square() * u,
        }
    }
}
