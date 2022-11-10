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

    pub const R: [u64; 6] = [
        0x7609_0000_0002_fffd,
        0xebf4_000b_c40c_0002,
        0x5f48_9857_53c7_58ba,
        0x77ce_5853_7052_5745,
        0x5c07_1a97_a256_ec6d,
        0x15f6_5ec3_fa80_e493,
    ];

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
        [
            0x5cb3_8790_fd53_0c16,
            0x7817_fc67_9976_fff5,
            0x154f_95c7_143b_a1c1,
            0xf0ae_6acd_f3d0_e747,
            0xedce_6ecc_21db_f440,
            0x1201_7741_9e0b_fb75,
        ],
        [
            0xbaac_93d5_0ce7_2271,
            0x8c22_631a_7918_fd8e,
            0xdd59_5f13_5707_25ce,
            0x51ac_5829_5040_5194,
            0x0e1c_8c3f_ad00_59c0,
            0x0bbc_3efc_5008_a26a,
        ],
        R,
    );

    const PARAM_A: [u64; 6] = [0, 0, 0, 0, 0, 0];

    const PARAM_B: [u64; 6] = [
        0xaa27_0000_000c_fff3,
        0x53cc_0032_fc34_000a,
        0x478f_e97a_6b0a_807f,
        0xb1d3_7ebe_e6ba_24d7,
        0x8ec9_733b_bf78_ab2f,
        0x09d6_4551_3d83_de7e,
    ];

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
