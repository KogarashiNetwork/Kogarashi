use crate::arithmetic::bits_256::limbs::*;
use crate::arithmetic::bits_256::represent::*;
use crate::arithmetic::utils::ProjectiveCoordinate;

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub fn add_point(
    lhs: ProjectiveCoordinate<[u64; 4]>,
    rhs: ProjectiveCoordinate<[u64; 4]>,
    p: [u64; 4],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 4]> {
    let zero: [u64; 4] = [0; 4];
    let (x, y, z) = lhs;
    let (a, b, c) = rhs;

    let s1 = mul(y, c, p, inv);
    let s2 = mul(b, z, p, inv);
    let u1 = mul(x, c, p, inv);
    let u2 = mul(a, z, p, inv);

    if u1 == u2 {
        if s1 == s2 {
            return double_point(lhs, p, inv);
        } else {
            return (zero, zero, zero);
        }
    }

    let s = sub(s1, s2, p);
    let u = sub(u1, u2, p);
    let uu = square(u, p, inv);
    let v = mul(z, c, p, inv);
    let w = sub(
        mul(square(s, p, inv), v, p, inv),
        mul(uu, add(u1, u2, p), p, inv),
        p,
    );
    let uuu = mul(uu, u, p, inv);

    (
        mul(u, w, p, inv),
        sub(
            mul(s, sub(mul(u1, uu, p, inv), w, p), p, inv),
            mul(s1, uuu, p, inv),
            p,
        ),
        mul(uuu, v, p, inv),
    )
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0, b = 4
pub fn double_point(
    rhs: ProjectiveCoordinate<[u64; 4]>,
    p: [u64; 4],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 4]> {
    let zero: [u64; 4] = [0; 4];
    let (x, y, z) = rhs;

    if z == zero || y == zero {
        (zero, zero, zero)
    } else {
        let xx = square(x, p, inv);
        let t = add(double(xx, p), xx, p);
        let u = double(mul(y, z, p, inv), p);
        let v = double(mul(mul(u, x, p, inv), y, p, inv), p);
        let w = sub(square(t, p, inv), double(v, p), p);
        let uu = square(u, p, inv);
        let yy = square(y, p, inv);
        let tvw = mul(t, sub(v, w, p), p, inv);
        (
            mul(u, w, p, inv),
            sub(tvw, double(mul(uu, yy, p, inv), p), p),
            mul(square(u, p, inv), u, p, inv),
        )
    }
}

pub fn scalar_point(
    mut base: ProjectiveCoordinate<[u64; 4]>,
    scalar: [u64; 4],
    mut identity: ProjectiveCoordinate<[u64; 4]>,
    p: [u64; 4],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 4]> {
    let bits = to_bits(scalar);
    for &bit in bits.iter() {
        if bit == 1 {
            identity = add_point(identity, base, p, inv);
        }
        base = double_point(base, p, inv);
    }
    identity
}
