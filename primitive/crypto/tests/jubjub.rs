use rand_core::RngCore;

pub mod field {
    use super::*;
    use zero_crypto::arithmetic::limbs::bits_256::*;

    pub const MODULUS: [u64; 4] = [
        0xd0970e5ed6f72cb7,
        0xa6682093ccc81082,
        0x06673b0101343b00,
        0x0e7db4ea6533afa9,
    ];

    pub const INV: u64 = 0x1ba3a358ef788ef9;

    const R2: [u64; 4] = [
        0x67719aa495e57731,
        0x51b0cef09ce3fc26,
        0x69dab7fac026e9a5,
        0x04f6547b8d127688,
    ];

    const R3: [u64; 4] = [
        0xe0d6c6563d830544,
        0x323e3883598d0f85,
        0xf0fea3004c2e2ba8,
        0x05874f84946737ec,
    ];

    pub fn random(rand: impl RngCore) -> [u64; 4] {
        random_limbs(rand, R2, R3, MODULUS, INV)
    }

    pub const fn from_raw(val: [u64; 4]) -> [u64; 4] {
        to_mont_form(val, R2, MODULUS, INV)
    }
}

pub mod curve {
    use super::field::*;
    use super::*;
    use zero_crypto::arithmetic::{
        coordinate::projective::{scalar_point, ProjectiveCoordinate},
        limbs::bits_256::*,
    };

    pub const IDENTITY: ProjectiveCoordinate = ([0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]);

    pub const GENERATOR: ProjectiveCoordinate = (
        from_raw([
            0x7c24d812779a3316,
            0x72e38f4ebd4070f3,
            0x03b3fe93f505a6f2,
            0xc4c71e5a4102960,
        ]),
        from_raw([
            0xd2047ef3463de4af,
            0x01ca03640d236cbf,
            0xd3033593ae386e92,
            0xaa87a50921b80ec,
        ]),
        from_raw([1, 0, 0, 0]),
    );

    const PARAM_A: [u64; 4] = [0, 0, 0, 0];

    const PARAM_B: [u64; 4] = from_raw([4, 0, 0, 0]);

    pub fn is_on_curve(point: ProjectiveCoordinate) -> bool {
        let identity = [0, 0, 0, 0];
        let (x, y, z) = point;

        if z == identity {
            true
        } else {
            let yy = square(y, MODULUS, INV);
            let right = mul(yy, z, MODULUS, INV);

            let xx = square(x, MODULUS, INV);
            let xxx = mul(xx, x, MODULUS, INV);
            let zz = square(z, MODULUS, INV);
            let zzz = mul(zz, z, MODULUS, INV);
            let c = mul(PARAM_B, zzz, MODULUS, INV);
            let left = add(xxx, c, MODULUS);

            right == left
        }
    }

    pub fn random_point(rand: impl RngCore) -> ProjectiveCoordinate {
        let random_scalar = random(rand);
        scalar_point(GENERATOR, random_scalar, IDENTITY, MODULUS, INV)
    }
}
