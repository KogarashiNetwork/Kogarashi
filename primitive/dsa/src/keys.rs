use zero_jubjub::{Fp, JubJubAffine, GENERATOR_EXTENDED};

struct KeyPair {
    private_key: Fp,
    pub public_key: JubJubAffine,
}

impl KeyPair {
    fn new(private_key: Fp) -> Self {
        let public_key = JubJubAffine::from(GENERATOR_EXTENDED * private_key);

        Self {
            private_key,
            public_key,
        }
    }
}
