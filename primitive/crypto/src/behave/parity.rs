// This trait resresents parity codec trait

use parity_scale_codec::{Decode, Encode};

/// This is parity compatible pallet
pub trait ParityCmp: Decode + Encode {}
