use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub(crate) struct Fr(pub(crate) [u64; 6]);
