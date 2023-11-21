use blake2b_simd::{Params, State};
use zkstd::common::PrimeField;

const MIMC_PERSONAL: &[u8; 4] = b"mimc";

pub(crate) struct BlakeHelper(State);

impl Default for BlakeHelper {
    fn default() -> Self {
        let state = Params::new()
            .hash_length(64)
            .personal(MIMC_PERSONAL)
            .to_state();

        Self(state)
    }
}

impl BlakeHelper {
    pub(crate) fn get(&self) -> [u8; 64] {
        *self.0.finalize().as_array()
    }

    pub(crate) fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    pub(crate) fn finalize<F: PrimeField>(&self) -> F {
        let digest = self.0.finalize();
        F::from_bytes_wide(digest.as_array())
    }
}
