// This trait represents compatibility trait

use parity_scale_codec::{Decode, Encode};

/// This is parity compatible pallet
pub trait ParityCmp: Decode + Encode {}

/// This is parallelize compatible pallet
pub trait ParallelCmp: Send + Sync {}
