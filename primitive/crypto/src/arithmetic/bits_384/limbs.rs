mod normal;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
pub use normal::{add, double, invert, mul, neg, square, sub};

#[cfg(any(not(feature = "asm"), not(target_arch = "x86_64")))]
pub use normal::{add, double, invert, mul, neg, square, sub};
