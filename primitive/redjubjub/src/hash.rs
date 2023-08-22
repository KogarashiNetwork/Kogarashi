use super::constant::SAPLING_PERSONAL;

use blake2b_simd::{Params, State};
use jub_jub::Fp;

pub(crate) fn sapling_hash(a: &[u8], b: &[u8], c: &[u8]) -> Fp {
    SaplingHash::default()
        .update(a)
        .update(b)
        .update(c)
        .finalize()
}

struct SaplingHash(State);

impl Default for SaplingHash {
    fn default() -> Self {
        let state = Params::new()
            .hash_length(64)
            .personal(SAPLING_PERSONAL)
            .to_state();

        Self(state)
    }
}

impl SaplingHash {
    pub(crate) fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    pub(crate) fn finalize(&self) -> Fp {
        let digest = self.0.finalize();
        Fp::from_hash(digest.as_array())
    }
}
