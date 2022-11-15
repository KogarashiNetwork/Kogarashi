use crate::arithmetic::bits_384::*;
use crate::arithmetic::utils::*;

/// The projective coordinate addition
/// cost: 11M + 6S + 1*a + 10add + 4*2 + 1*4.
/// a = 0
pub fn add_point(
    lhs: ProjectiveCoordinate<[u64; 6]>,
    rhs: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    let zero: [u64; 6] = [0; 6];
    let (x, y, z) = lhs;
    let (a, b, c) = rhs;

    let s1 = mul(y, c, p, inv);
    let s2 = mul(b, z, p, inv);
    let u1 = mul(x, c, p, inv);
    let u2 = mul(a, z, p, inv);

    if u1 == u2 {
        if s1 == s2 {
            double_point(lhs, p, inv)
        } else {
            (zero, zero, zero)
        }
    } else {
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
                mul(s, sub(mul(u1, uu, p, inv), s1, p), p, inv),
                mul(s1, uuu, p, inv),
                p,
            ),
            mul(uuu, v, p, inv),
        )
    }
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + 7add + 3*2 + 1*3.
/// a = 0
pub fn double_point(
    rhs: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    let zero: [u64; 6] = [0; 6];
    let (x, y, z) = rhs;

    if z == zero || y == zero {
        (zero, zero, zero)
    } else {
        let xx = square(x, p, inv);
        let w = add(xx, double(xx, p), p);
        let s = double(mul(y, z, p, inv), p);
        let ss = square(s, p, inv);
        let sss = mul(s, ss, p, inv);
        let r = mul(y, s, p, inv);
        let rr = square(r, p, inv);
        let b = sub(sub(square(add(x, r, p), p, inv), xx, p), rr, p);
        let h = sub(square(w, p, inv), double(b, p), p);

        (
            mul(h, s, p, inv),
            sub(mul(w, sub(b, h, p), p, inv), double(rr, p), p),
            sss,
        )
    }
}

pub fn scalar_point(
    mut base: ProjectiveCoordinate<[u64; 6]>,
    scalar: [u64; 6],
    mut identity: ProjectiveCoordinate<[u64; 6]>,
    p: [u64; 6],
    inv: u64,
) -> ProjectiveCoordinate<[u64; 6]> {
    let bits = to_bits(scalar);
    for &bit in bits.iter() {
        if bit == 1 {
            identity = add_point(identity, base, p, inv);
        }
        base = double_point(base, p, inv);
    }
    identity
}
