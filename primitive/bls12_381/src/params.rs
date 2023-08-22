//! Bls12 381 construction and frobennius map constant
use crate::{Fq, Fq2, Fr};

pub const BLS_X: u64 = 0xd201000000010000;
pub const BLS_X_IS_NEGATIVE: bool = true;

pub const EDWARDS_D: Fr = Fr([
    0x01065fd6d6343eb1,
    0x292d7f6d37579d26,
    0xf5fd9207e6bd7fd4,
    0x2a9318e74bfa2b48,
]);

// g1 curve parameters
pub(crate) const G1_GENERATOR_X: Fq = Fq([
    0x5cb38790fd530c16,
    0x7817fc679976fff5,
    0x154f95c7143ba1c1,
    0xf0ae6acdf3d0e747,
    0xedce6ecc21dbf440,
    0x120177419e0bfb75,
]);
pub(crate) const G1_GENERATOR_Y: Fq = Fq([
    0xbaac93d50ce72271,
    0x8c22631a7918fd8e,
    0xdd595f13570725ce,
    0x51ac582950405194,
    0x0e1c8c3fad0059c0,
    0x0bbc3efc5008a26a,
]);
pub(crate) const G1_PARAM_A: Fq = Fq([0, 0, 0, 0, 0, 0]);
pub(crate) const G1_PARAM_B: Fq = Fq([
    0xaa270000000cfff3,
    0x53cc0032fc34000a,
    0x478fe97a6b0a807f,
    0xb1d37ebee6ba24d7,
    0x8ec9733bbf78ab2f,
    0x09d645513d83de7e,
]);

// g2 curve parameters
pub(crate) const G2_GENERATOR_X: Fq2 = Fq2([
    Fq([
        0xf5f28fa202940a10,
        0xb3f5fb2687b4961a,
        0xa1a893b53e2ae580,
        0x9894999d1a3caee9,
        0x6f67b7631863366b,
        0x058191924350bcd7,
    ]),
    Fq([
        0xa5a9c0759e23f606,
        0xaaa0c59dbccd60c3,
        0x3bb17e18e2867806,
        0x1b1ab6cc8541b367,
        0xc2b6ed0ef2158547,
        0x11922a097360edf3,
    ]),
]);
pub(crate) const G2_GENERATOR_Y: Fq2 = Fq2([
    Fq([
        0x4c730af860494c4a,
        0x597cfa1f5e369c5a,
        0xe7e6856caa0a635a,
        0xbbefb5e96e0d495f,
        0x07d3a975f0ef25a2,
        0x0083fd8e7e80dae5,
    ]),
    Fq([
        0xadc0fc92df64b05d,
        0x18aa270a2b1461dc,
        0x86adac6a3be4eba0,
        0x79495c4ec93da33a,
        0xe7175850a43ccaed,
        0x0b2bc2a163de1bf2,
    ]),
]);
pub(crate) const G2_PARAM_A: Fq2 = Fq2([Fq([0, 0, 0, 0, 0, 0]), Fq([0, 0, 0, 0, 0, 0])]);
pub(crate) const G2_PARAM_B: Fq2 = Fq2([
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x09d645513d83de7e,
    ]),
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x09d645513d83de7e,
    ]),
]);

pub(crate) const FROBENIUS_COEFF_FQ2_C1: [Fq; 2] = [
    // Fq(-1)**(((q^0) - 1) / 2)
    Fq([
        0x760900000002fffd,
        0xebf4000bc40c0002,
        0x5f48985753c758ba,
        0x77ce585370525745,
        0x5c071a97a256ec6d,
        0x15f65ec3fa80e493,
    ]),
    // Fq(-1)**(((q^1) - 1) / 2)
    Fq([
        0x43f5fffffffcaaae,
        0x32b7fff2ed47fffd,
        0x7e83a49a2e99d69,
        0xeca8f3318332bb7a,
        0xef148d1ea0f4c069,
        0x40ab3263eff0206,
    ]),
];

pub(crate) const FROBENIUS_COEFF_FQ6_C1: [Fq2; 6] = [
    // Fq2(u + 1)**(((q^0) - 1) / 3)
    Fq2([
        Fq([
            0x760900000002fffd,
            0xebf4000bc40c0002,
            0x5f48985753c758ba,
            0x77ce585370525745,
            0x5c071a97a256ec6d,
            0x15f65ec3fa80e493,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^1) - 1) / 3)
    Fq2([
        Fq::zero(),
        Fq([
            0xcd03c9e48671f071,
            0x5dab22461fcda5d2,
            0x587042afd3851b95,
            0x8eb60ebe01bacb9e,
            0x3f97d6e83d050d2,
            0x18f0206554638741,
        ]),
    ]),
    // Fq2(u + 1)**(((q^2) - 1) / 3)
    Fq2([
        Fq([
            0x30f1361b798a64e8,
            0xf3b8ddab7ece5a2a,
            0x16a8ca3ac61577f7,
            0xc26a2ff874fd029b,
            0x3636b76660701c6e,
            0x51ba4ab241b6160,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^3) - 1) / 3)
    Fq2([
        Fq::zero(),
        Fq([
            0x760900000002fffd,
            0xebf4000bc40c0002,
            0x5f48985753c758ba,
            0x77ce585370525745,
            0x5c071a97a256ec6d,
            0x15f65ec3fa80e493,
        ]),
    ]),
    // Fq2(u + 1)**(((q^4) - 1) / 3)
    Fq2([
        Fq([
            0xcd03c9e48671f071,
            0x5dab22461fcda5d2,
            0x587042afd3851b95,
            0x8eb60ebe01bacb9e,
            0x3f97d6e83d050d2,
            0x18f0206554638741,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^5) - 1) / 3)
    Fq2([
        Fq::zero(),
        Fq([
            0x30f1361b798a64e8,
            0xf3b8ddab7ece5a2a,
            0x16a8ca3ac61577f7,
            0xc26a2ff874fd029b,
            0x3636b76660701c6e,
            0x51ba4ab241b6160,
        ]),
    ]),
];

pub const FROBENIUS_COEFF_FQ6_C2: [Fq2; 6] = [
    // Fq2(u + 1)**(((2q^0) - 2) / 3)
    Fq2([
        Fq([
            0x760900000002fffd,
            0xebf4000bc40c0002,
            0x5f48985753c758ba,
            0x77ce585370525745,
            0x5c071a97a256ec6d,
            0x15f65ec3fa80e493,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((2q^1) - 2) / 3)
    Fq2([
        Fq([
            0x890dc9e4867545c3,
            0x2af322533285a5d5,
            0x50880866309b7e2c,
            0xa20d1b8c7e881024,
            0x14e4f04fe2db9068,
            0x14e56d3f1564853a,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((2q^2) - 2) / 3)
    Fq2([
        Fq([
            0xcd03c9e48671f071,
            0x5dab22461fcda5d2,
            0x587042afd3851b95,
            0x8eb60ebe01bacb9e,
            0x3f97d6e83d050d2,
            0x18f0206554638741,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((2q^3) - 2) / 3)
    Fq2([
        Fq([
            0x43f5fffffffcaaae,
            0x32b7fff2ed47fffd,
            0x7e83a49a2e99d69,
            0xeca8f3318332bb7a,
            0xef148d1ea0f4c069,
            0x40ab3263eff0206,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((2q^4) - 2) / 3)
    Fq2([
        Fq([
            0x30f1361b798a64e8,
            0xf3b8ddab7ece5a2a,
            0x16a8ca3ac61577f7,
            0xc26a2ff874fd029b,
            0x3636b76660701c6e,
            0x51ba4ab241b6160,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((2q^5) - 2) / 3)
    Fq2([
        Fq([
            0xecfb361b798dba3a,
            0xc100ddb891865a2c,
            0xec08ff1232bda8e,
            0xd5c13cc6f1ca4721,
            0x47222a47bf7b5c04,
            0x110f184e51c5f59,
        ]),
        Fq::zero(),
    ]),
];

pub(crate) const FROBENIUS_COEFF_FQ12_C1: [Fq2; 12] = [
    // Fq2(u + 1)**(((q^0) - 1) / 6)
    Fq2([
        Fq([
            0x760900000002fffd,
            0xebf4000bc40c0002,
            0x5f48985753c758ba,
            0x77ce585370525745,
            0x5c071a97a256ec6d,
            0x15f65ec3fa80e493,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^1) - 1) / 6)
    Fq2([
        Fq([
            0x7089552b319d465,
            0xc6695f92b50a8313,
            0x97e83cccd117228f,
            0xa35baecab2dc29ee,
            0x1ce393ea5daace4d,
            0x8f2220fb0fb66eb,
        ]),
        Fq([
            0xb2f66aad4ce5d646,
            0x5842a06bfc497cec,
            0xcf4895d42599d394,
            0xc11b9cba40a8e8d0,
            0x2e3813cbe5a0de89,
            0x110eefda88847faf,
        ]),
    ]),
    // Fq2(u + 1)**(((q^2) - 1) / 6)
    Fq2([
        Fq([
            0xecfb361b798dba3a,
            0xc100ddb891865a2c,
            0xec08ff1232bda8e,
            0xd5c13cc6f1ca4721,
            0x47222a47bf7b5c04,
            0x110f184e51c5f59,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^3) - 1) / 6)
    Fq2([
        Fq([
            0x3e2f585da55c9ad1,
            0x4294213d86c18183,
            0x382844c88b623732,
            0x92ad2afd19103e18,
            0x1d794e4fac7cf0b9,
            0xbd592fc7d825ec8,
        ]),
        Fq([
            0x7bcfa7a25aa30fda,
            0xdc17dec12a927e7c,
            0x2f088dd86b4ebef1,
            0xd1ca2087da74d4a7,
            0x2da2596696cebc1d,
            0xe2b7eedbbfd87d2,
        ]),
    ]),
    // Fq2(u + 1)**(((q^4) - 1) / 6)
    Fq2([
        Fq([
            0x30f1361b798a64e8,
            0xf3b8ddab7ece5a2a,
            0x16a8ca3ac61577f7,
            0xc26a2ff874fd029b,
            0x3636b76660701c6e,
            0x51ba4ab241b6160,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^5) - 1) / 6)
    Fq2([
        Fq([
            0x3726c30af242c66c,
            0x7c2ac1aad1b6fe70,
            0xa04007fbba4b14a2,
            0xef517c3266341429,
            0x95ba654ed2226b,
            0x2e370eccc86f7dd,
        ]),
        Fq([
            0x82d83cf50dbce43f,
            0xa2813e53df9d018f,
            0xc6f0caa53c65e181,
            0x7525cf528d50fe95,
            0x4a85ed50f4798a6b,
            0x171da0fd6cf8eebd,
        ]),
    ]),
    // Fq2(u + 1)**(((q^6) - 1) / 6)
    Fq2([
        Fq([
            0x43f5fffffffcaaae,
            0x32b7fff2ed47fffd,
            0x7e83a49a2e99d69,
            0xeca8f3318332bb7a,
            0xef148d1ea0f4c069,
            0x40ab3263eff0206,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^7) - 1) / 6)
    Fq2([
        Fq([
            0xb2f66aad4ce5d646,
            0x5842a06bfc497cec,
            0xcf4895d42599d394,
            0xc11b9cba40a8e8d0,
            0x2e3813cbe5a0de89,
            0x110eefda88847faf,
        ]),
        Fq([
            0x7089552b319d465,
            0xc6695f92b50a8313,
            0x97e83cccd117228f,
            0xa35baecab2dc29ee,
            0x1ce393ea5daace4d,
            0x8f2220fb0fb66eb,
        ]),
    ]),
    // Fq2(u + 1)**(((q^8) - 1) / 6)
    Fq2([
        Fq([
            0xcd03c9e48671f071,
            0x5dab22461fcda5d2,
            0x587042afd3851b95,
            0x8eb60ebe01bacb9e,
            0x3f97d6e83d050d2,
            0x18f0206554638741,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^9) - 1) / 6)
    Fq2([
        Fq([
            0x7bcfa7a25aa30fda,
            0xdc17dec12a927e7c,
            0x2f088dd86b4ebef1,
            0xd1ca2087da74d4a7,
            0x2da2596696cebc1d,
            0xe2b7eedbbfd87d2,
        ]),
        Fq([
            0x3e2f585da55c9ad1,
            0x4294213d86c18183,
            0x382844c88b623732,
            0x92ad2afd19103e18,
            0x1d794e4fac7cf0b9,
            0xbd592fc7d825ec8,
        ]),
    ]),
    // Fq2(u + 1)**(((q^10) - 1) / 6)
    Fq2([
        Fq([
            0x890dc9e4867545c3,
            0x2af322533285a5d5,
            0x50880866309b7e2c,
            0xa20d1b8c7e881024,
            0x14e4f04fe2db9068,
            0x14e56d3f1564853a,
        ]),
        Fq::zero(),
    ]),
    // Fq2(u + 1)**(((q^11) - 1) / 6)
    Fq2([
        Fq([
            0x82d83cf50dbce43f,
            0xa2813e53df9d018f,
            0xc6f0caa53c65e181,
            0x7525cf528d50fe95,
            0x4a85ed50f4798a6b,
            0x171da0fd6cf8eebd,
        ]),
        Fq([
            0x3726c30af242c66c,
            0x7c2ac1aad1b6fe70,
            0xa04007fbba4b14a2,
            0xef517c3266341429,
            0x95ba654ed2226b,
            0x2e370eccc86f7dd,
        ]),
    ]),
];
