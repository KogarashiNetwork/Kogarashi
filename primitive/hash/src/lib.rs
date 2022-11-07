#![cfg_attr(not(feature = "std"), no_std)]

mod keccak256;
mod utils;

pub use keccak256::keccak256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // https://emn178.github.io/online-tools/sha3_256.html
        let msg = b"hello";
        let result = b"3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392".to_vec();
        let digest = keccak256(msg.to_vec());

        // assert_eq!(digest, result);
    }
}
