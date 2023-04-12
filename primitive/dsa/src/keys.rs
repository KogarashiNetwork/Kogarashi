use zero_crypto::common::CurveGroup;
use zero_jubjub::{Fp, JubjubAffine, JubjubExtend};

struct KeyPair {
    private_key: Fp,
    pub public_key: JubjubAffine,
}

impl KeyPair {
    fn new(private_key: Fp) -> Self {
        let public_key = JubjubAffine::from(JubjubExtend::ADDITIVE_GENERATOR * private_key);

        Self {
            private_key,
            public_key,
        }
    }
}
