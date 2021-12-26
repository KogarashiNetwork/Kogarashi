#![cfg_attr(not(feature = "std"), no_std)]
#![feature(asm)]
#![allow(dead_code)]

mod arithmetic;
mod error;
mod fr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
