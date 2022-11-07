#![cfg_attr(not(feature = "std"), no_std)]

mod kzg;
mod poly;

pub use crate::kzg::Kzg;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
