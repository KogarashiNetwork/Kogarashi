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
