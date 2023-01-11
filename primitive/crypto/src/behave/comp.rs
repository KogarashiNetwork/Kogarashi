// trait represents compatibility trait
use core::fmt::Debug;
use parity_scale_codec::{Decode, Encode};

/// parallelize compatible pallet
pub trait ParallelCmp: Send + Sync {}

/// parity compatible pallet
pub trait ParityCmp: Decode + Encode {}

/// substrate runtime pallet
pub trait RuntimeCmp: Send + Sync + Sized + Eq + PartialEq + Clone + 'static {}

/// basic struct trait
pub trait Basic: Clone + Copy + Debug + Default + Sized {}
