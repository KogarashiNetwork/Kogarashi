//! Bls12 381 construction and frobennius map constant
use bn_254::{Fq, Fq2};

pub const BLS_X: u64 = 4965661367192848881;
pub const BLS_X_IS_NEGATIVE: bool = true;

// g1 curve parameters
pub(crate) const G1_GENERATOR_X: Fq = Fq::one();
pub(crate) const G1_GENERATOR_Y: Fq = Fq::new_unchecked([
    0x11b2dff1448c41d8,
    0x23d3446f21c77dc3,
    0xaa7b8cf435dfafbb,
    0x14b34cf69dc25d68,
]);
pub(crate) const G1_PARAM_B: Fq = Fq::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);

// g2 curve parameters
pub(crate) const G2_GENERATOR_X: Fq2 = Fq2::new_unchecked([
    Fq::to_mont_form([
        0x46debd5cd992f6ed,
        0x674322d4f75edadd,
        0x426a00665e5c4479,
        0x1800deef121f1e76,
    ]),
    Fq::to_mont_form([
        0x97e485b7aef312c2,
        0xf1aa493335a9e712,
        0x7260bfb731fb5d25,
        0x198e9393920d483a,
    ]),
]);
pub(crate) const G2_GENERATOR_Y: Fq2 = Fq2::new_unchecked([
    Fq::to_mont_form([
        0x4ce6cc0166fa7daa,
        0xe3d1e7690c43d37b,
        0x4aab71808dcb408f,
        0x12c85ea5db8c6deb,
    ]),
    Fq::to_mont_form([
        0x55acdadcd122975b,
        0xbc4b313370b38ef3,
        0xec9e99ad690c3395,
        0x090689d0585ff075,
    ]),
]);
pub(crate) const G2_PARAM_A: Fq2 = Fq2::new_unchecked([Fq::zero(), Fq::zero()]);
pub(crate) const G2_PARAM_B: Fq2 = Fq2::new_unchecked([
    Fq::to_mont_form([
        0x3267e6dc24a138e5,
        0xb5b4c5e559dbefa3,
        0x81be18991be06ac3,
        0x2b149d40ceb8aaae,
    ]),
    Fq::to_mont_form([
        0xe4a2bd0685c315d2,
        0xa74fa084e52d1852,
        0xcd2cafadeed8fdf4,
        0x009713b03af0fed4,
    ]),
]);

pub(crate) const FROBENIUS_COEFF_FQ2_C1: [Fq; 2] = [
    // Fq::new_unchecked(-1)**(((q^0) - 1) / 2)
    Fq::new_unchecked([
        0xd35d438dc58f0d9d,
        0x0a78eb28f5c70b3d,
        0x666ea36f7879462c,
        0x0e0a77c19a07df2f,
    ]),
    // Fq::new_unchecked(-1)**(((q^1) - 1) / 2)
    Fq::new_unchecked([
        0x68c3488912edefaa,
        0x8d087f6872aabf4f,
        0x51e1a24709081231,
        0x2259d6b14729c0fa,
    ]),
];

pub(crate) const FROBENIUS_COEFF_FQ6_C1: [Fq2; 6] = [
    // Fq2::new_unchecked(u + 9)**(((q^0) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xd35d438dc58f0d9d,
            0x0a78eb28f5c70b3d,
            0x666ea36f7879462c,
            0x0e0a77c19a07df2f,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^1) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xb5773b104563ab30,
            0x347f91c8a9aa6454,
            0x7a007127242e0991,
            0x1956bcd8118214ec,
        ]),
        Fq::new_unchecked([
            0x6e849f1ea0aa4757,
            0xaa1c7b6d89f89141,
            0xb6e713cdfae0ca3a,
            0x26694fbb4e82ebc3,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^2) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x3350c88e13e80b9c,
            0x7dce557cdb5e56b9,
            0x6001b4b8b615564a,
            0x2682e617020217e0,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^3) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xc9af22f716ad6bad,
            0xb311782a4aa662b2,
            0x19eeaf64e248c7f4,
            0x20273e77e3439f82,
        ]),
        Fq::new_unchecked([
            0xacc02860f7ce93ac,
            0x3933d5817ba76b4c,
            0x69e6188b446c8467,
            0x0a46036d4417cc55,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^4) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x71930c11d782e155,
            0xa6bb947cffbe3323,
            0xaa303344d4741444,
            0x2c3b3f0d26594943,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^5) - 1) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xf91aba2654e8e3b1,
            0x4771cb2fdc92ce12,
            0xdcb16ae0fc8bdf35,
            0x274aa195cd9d8be4,
        ]),
        Fq::new_unchecked([
            0x5cfc50ae18811f8b,
            0x4bb28433cb43988c,
            0x4fd35f13c3b56219,
            0x301949bd2fc8883a,
        ]),
    ]),
];

pub const FROBENIUS_COEFF_FQ6_C2: [Fq2; 6] = [
    // Fq2::new_unchecked(u + 1)**(((2q^0) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xd35d438dc58f0d9d,
            0x0a78eb28f5c70b3d,
            0x666ea36f7879462c,
            0x0e0a77c19a07df2f,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 1)**(((2q^1) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x7361d77f843abe92,
            0xa5bb2bd3273411fb,
            0x9c941f314b3e2399,
            0x15df9cddbb9fd3ec,
        ]),
        Fq::new_unchecked([
            0x5dddfd154bd8c949,
            0x62cb29a5a4445b60,
            0x37bc870a0c7dd2b9,
            0x24830a9d3171f0fd,
        ]),
    ]),
    // Fq2::new_unchecked(u + 1)**(((2q^2) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x71930c11d782e155,
            0xa6bb947cffbe3323,
            0xaa303344d4741444,
            0x2c3b3f0d26594943,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 1)**(((2q^3) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x448a93a57b6762df,
            0xbfd62df528fdeadf,
            0xd858f5d00e9bd47a,
            0x06b03d4d3476ec58,
        ]),
        Fq::new_unchecked([
            0x2b19daf4bcc936d1,
            0xa1a54e7a56f4299f,
            0xb533eee05adeaef1,
            0x170c812b84dda0b2,
        ]),
    ]),
    // Fq2::new_unchecked(u + 1)**(((2q^4) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x3350c88e13e80b9c,
            0x7dce557cdb5e56b9,
            0x6001b4b8b615564a,
            0x2682e617020217e0,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 1)**(((2q^5) - 2) / 3)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x843420f1d8dadbd6,
            0x31f010c9183fcdb2,
            0x436330b527a76049,
            0x13d47447f11adfe4,
        ]),
        Fq::new_unchecked([
            0xef494023a857fa74,
            0x2a925d02d5ab101a,
            0x83b015829ba62f10,
            0x2539111d0c13aea3,
        ]),
    ]),
];

pub(crate) const FROBENIUS_COEFF_FQ12_C1: [Fq2; 12] = [
    // Fq2::new_unchecked(u + 9)**(((q^0) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xd35d438dc58f0d9d,
            0x0a78eb28f5c70b3d,
            0x666ea36f7879462c,
            0x0e0a77c19a07df2f,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 1)**(((q^1) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xaf9ba69633144907,
            0xca6b1d7387afb78a,
            0x11bded5ef08a2087,
            0x02f34d751a1f3a7c,
        ]),
        Fq::new_unchecked([
            0xa222ae234c492d72,
            0xd00f02a4565de15b,
            0xdc2ff3a253dfc926,
            0x10a75716b3899551,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^2) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xca8d800500fa1bf2,
            0xf0c5d61468b39769,
            0x0e201271ad0d4418,
            0x04290f65bad856e6,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^3) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x365316184e46d97d,
            0x0af7129ed4c96d9f,
            0x659da72fca1009b5,
            0x08116d8983a20d23,
        ]),
        Fq::new_unchecked([
            0xb1df4af7c39c1939,
            0x3d9f02878a73bf7f,
            0x9b2220928caf0ae0,
            0x26684515eff054a6,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^4) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x3350c88e13e80b9c,
            0x7dce557cdb5e56b9,
            0x6001b4b8b615564a,
            0x2682e617020217e0,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^5) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x86b76f821b329076,
            0x408bf52b4d19b614,
            0x53dfb9d0d985e92d,
            0x051e20146982d2a7,
        ]),
        Fq::new_unchecked([
            0x0fbc9cd47752ebc7,
            0x6d8fffe33415de24,
            0xbef22cf038cf41b9,
            0x15c0edff3c66bf54,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^6) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x68c3488912edefaa,
            0x8d087f6872aabf4f,
            0x51e1a24709081231,
            0x2259d6b14729c0fa,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^7) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x8c84e580a568b440,
            0xcd164d1de0c21302,
            0xa692585790f737d5,
            0x2d7100fdc71265ad,
        ]),
        Fq::new_unchecked([
            0x99fdddf38c33cfd5,
            0xc77267ed1213e931,
            0xdc2052142da18f36,
            0x1fbcf75c2da80ad7,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^8) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x71930c11d782e155,
            0xa6bb947cffbe3323,
            0xaa303344d4741444,
            0x2c3b3f0d26594943,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^9) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x05cd75fe8a3623ca,
            0x8c8a57f293a85cee,
            0x52b29e86b7714ea8,
            0x2852e0e95d8f9306,
        ]),
        Fq::new_unchecked([
            0x8a41411f14e0e40e,
            0x59e26809ddfe0b0d,
            0x1d2e2523f4d24d7d,
            0x09fc095cf1414b83,
        ]),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^10) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0x08cfc388c494f1ab,
            0x19b315148d1373d4,
            0x584e90fdcb6c0213,
            0x09e1685bdf2f8849,
        ]),
        Fq::zero(),
    ]),
    // Fq2::new_unchecked(u + 9)**(((q^11) - 1) / 6)
    Fq2::new_unchecked([
        Fq::new_unchecked([
            0xb5691c94bd4a6cd1,
            0x56f575661b581478,
            0x64708be5a7fb6f30,
            0x2b462e5e77aecd82,
        ]),
        Fq::new_unchecked([
            0x2c63ef42612a1180,
            0x29f16aae345bec69,
            0xf95e18c648b216a4,
            0x1aa36073a4cae0d4,
        ]),
    ]),
];
