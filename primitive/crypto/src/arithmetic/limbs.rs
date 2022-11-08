mod utils;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
mod assembly;

#[cfg(any(not(feature = "asm"), not(target_arch = "x86_64")))]
mod normal;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
pub(crate) use assembly::{add, double, mul, neg, square, sub};

#[cfg(any(not(feature = "asm"), not(target_arch = "x86_64")))]
pub(crate) use normal::{add, double, mul, neg, square, sub};

pub(crate) use normal::invert;
