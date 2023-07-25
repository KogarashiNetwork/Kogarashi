use super::constant::{KOGARASHI_PERSONAL, SAPLING_PERSONAL};

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

pub fn kogarashi_hash(seed: &[u8]) -> Fp {
    KogarashiHash::default().update(seed).finalize()
}

struct KogarashiHash(State);

impl Default for KogarashiHash {
    fn default() -> Self {
        let state = Params::new()
            .hash_length(64)
            .personal(KOGARASHI_PERSONAL)
            .to_state();

        Self(state)
    }
}

impl KogarashiHash {
    pub(crate) fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    pub(crate) fn finalize(&self) -> Fp {
        let digest = self.0.finalize();
        Fp::from_hash(digest.as_array())
    }
}
