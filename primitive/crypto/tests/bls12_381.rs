use rand_core::RngCore;

pub mod field {
    use super::*;
    use zero_crypto::arithmetic::limbs::bits_384::*;

    pub const MODULUS: [u64; 6] = [
        0xb9fe_ffff_ffff_aaab,
        0x1eab_fffe_b153_ffff,
        0x6730_d2a0_f6b0_f624,
        0x6477_4b84_f385_12bf,
        0x4b1b_a7b6_434b_acd7,
        0x1a01_11ea_397f_e69a,
    ];

    pub const INV: u64 = 0x89f3_fffc_fffc_fffd;

    const R2: [u64; 6] = [
        0xf4df_1f34_1c34_1746,
        0x0a76_e6a6_09d1_04f1,
        0x8de5_476c_4c95_b6d5,
        0x67eb_88a9_939d_83c0,
        0x9a79_3e85_b519_952d,
        0x1198_8fe5_92ca_e3aa,
    ];

    const R3: [u64; 6] = [
        0xed48_ac6b_d94c_a1e0,
        0x315f_831e_03a7_adf8,
        0x9a53_352a_615e_29dd,
        0x34c0_4e5e_921e_1761,
        0x2512_d435_6572_4728,
        0x0aa6_3460_9175_5d4d,
    ];

    pub fn random(rand: impl RngCore) -> [u64; 6] {
        random_limbs(rand, R2, R3, MODULUS, INV)
    }

    pub const fn from_raw(val: [u64; 6]) -> [u64; 6] {
        to_mont_form(val, R2, MODULUS, INV)
    }
}

pub mod curve {
    use super::field::*;
    use super::*;
    use zero_crypto::arithmetic::{
        coordinate::bits_384::projective::*, coordinate::utils::*, limbs::bits_384::*,
    };

    pub const IDENTITY: ProjectiveCoordinate<[u64; 6]> =
        ([0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]);

    pub const GENERATOR: ProjectiveCoordinate<[u64; 6]> = (
        from_raw([
            0x5cb3_8790_fd53_0c16,
            0x7817_fc67_9976_fff5,
            0x154f_95c7_143b_a1c1,
            0xf0ae_6acd_f3d0_e747,
            0xedce_6ecc_21db_f440,
            0x1201_7741_9e0b_fb75,
        ]),
        from_raw([
            0x5cb3_8790_fd53_0c16,
            0x7817_fc67_9976_fff5,
            0x154f_95c7_143b_a1c1,
            0xf0ae_6acd_f3d0_e747,
            0xedce_6ecc_21db_f440,
            0x1201_7741_9e0b_fb75,
        ]),
        from_raw([1, 0, 0, 0, 0, 0]),
    );

    const PARAM_A: [u64; 6] = [0, 0, 0, 0, 0, 0];

    const PARAM_B: [u64; 6] = from_raw([4, 0, 0, 0, 0, 0]);

    pub fn is_on_curve(point: ProjectiveCoordinate<[u64; 6]>) -> bool {
        let identity = [0, 0, 0, 0, 0, 0];
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

    pub fn random_point(rand: impl RngCore) -> ProjectiveCoordinate<[u64; 6]> {
        let random_scalar = random(rand);
        scalar_point(GENERATOR, random_scalar, IDENTITY, MODULUS, INV)
    }
}
