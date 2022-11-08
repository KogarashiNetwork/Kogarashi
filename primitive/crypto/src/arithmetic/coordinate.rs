use crate::arithmetic::limbs::*;

type ProjectiveCoordinate = ([u64; 4], [u64; 4], [u64; 4]);

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub fn add_point(
    lhs: ProjectiveCoordinate,
    rhs: ProjectiveCoordinate,
    p: [u64; 4],
) -> ProjectiveCoordinate {
    let (x, y, z) = lhs;
    let (a, b, c) = rhs;
    let zero: [u64; 4] = [0; 4];

    if z == zero {
        return rhs;
    } else if c == zero {
        return lhs;
    }

    let s1 = mul(y, c, p);
    let s2 = mul(b, z, p);
    let u1 = mul(x, c, p);
    let u2 = mul(a, z, p);

    if u1 == u2 {
        let identity = (zero, zero, zero);
        if s1 == s2 {
            double_point(lhs, p)
        } else {
            identity
        }
    } else {
        let s = sub(s1, s2, p);
        let u = sub(u1, u2, p);
        let uu = square(u, p);
        let v = mul(z, c, p);
        let ss = square(s, p);
        let ssv = mul(ss, v, p);
        let u1u2 = add(u1, u2, p);
        let uuu1u2 = mul(uu, u1u2, p);
        let w = sub(ssv, uuu1u2, p);
        let uuu = mul(uu, u, p);

        let u1uu = mul(u1, uu, p);
        let wu1uu = sub(u1uu, w, p);
        let swu1uu = mul(s, wu1uu, p);
        let s1uuu = mul(s1, uuu, p);

        (mul(u, w, p), mul(swu1uu, s1uuu, p), mul(uuu, v, p))
    }
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0, b = 4
pub(crate) fn double_point(rhs: ProjectiveCoordinate, p: [u64; 4]) -> ProjectiveCoordinate {
    let (x, y, z) = rhs;
    let zero: [u64; 4] = [0; 4];
    let identity = (zero, zero, zero);

    if z == zero || y == zero {
        identity
    } else {
        let xx = square(x, p);
        let t = add(double(xx, p), xx, p);
        let u = double(mul(y, z, p), p);
        let v = double(mul(mul(u, x, p), y, p), p);
        let w = sub(square(t, p), double(v, p), p);
        let uu = square(u, p);
        let yy = square(y, p);
        let tvw = mul(t, sub(v, w, p), p);
        (
            mul(u, w, p),
            sub(tvw, double(mul(uu, yy, p), p), p),
            mul(square(u, p), u, p),
        )
    }
}
