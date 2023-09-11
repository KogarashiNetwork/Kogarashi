use super::constant::SAPLING_PERSONAL;

use blake2b_simd::{Params, State};
use zkstd::behave::Group;
use zkstd::common::Pairing;

pub fn sapling_hash<P: Pairing>(a: &[u8], b: &[u8], c: &[u8]) -> P::JubjubScalar {
    SaplingHash::default()
        .update(a)
        .update(b)
        .update(c)
        .finalize::<P>()
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

    pub(crate) fn finalize<P: Pairing>(&self) -> P::JubjubScalar {
        let digest = self.0.finalize();
        // P::JubjubScalar::from_hash(digest.as_array())
        P::JubjubScalar::zero()
    }
}
