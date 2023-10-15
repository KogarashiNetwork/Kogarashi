use zkstd::common::{Pairing, RngCore};

/// public parameters trait
pub trait PublicParameters<P: Pairing> {
    const ADDITIONAL_DEGREE: usize;

    fn setup(k: u64, r: impl RngCore) -> Self;
}
