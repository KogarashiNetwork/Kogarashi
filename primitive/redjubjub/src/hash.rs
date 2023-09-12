use super::constant::SAPLING_PERSONAL;

use blake2b_simd::{Params, State};
use zkstd::common::FftField;

pub fn sapling_hash<F: FftField>(a: &[u8], b: &[u8], c: &[u8]) -> F {
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

    pub(crate) fn finalize<F: FftField>(&self) -> F {
        let digest = self.0.finalize();
        F::from_hash(digest.as_array())
    }
}
