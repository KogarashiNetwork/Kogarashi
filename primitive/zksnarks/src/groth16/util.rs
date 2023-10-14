#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::borrow::Borrow;

/// Like SliceConcatExt::join for strings, but works in stable with no_std.
/// See https://github.com/rust-lang/rust/issues/27747
pub fn join<S: Borrow<str>>(sep: &str, strings: &[S]) -> String {
    let mut builder = String::new();
    for s in strings {
        if !builder.is_empty() {
            builder += sep;
        }
        builder += s.borrow();
    }
    builder
}
