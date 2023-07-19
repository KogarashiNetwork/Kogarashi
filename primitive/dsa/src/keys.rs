use zero_crypto::common::CurveGroup;
use zero_jubjub::{Fp, JubjubAffine, JubjubExtended};

struct KeyPair {
    private_key: Fp,
    pub public_key: JubjubAffine,
}

impl KeyPair {
    fn new(private_key: Fp) -> Self {
        let public_key = JubjubAffine::from(JubjubExtended::ADDITIVE_GENERATOR * private_key);

        Self {
            private_key,
            public_key,
        }
    }
}
