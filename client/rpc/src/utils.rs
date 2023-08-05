use hex::encode;
use std::vec;

use sp_core::hexdisplay::AsBytesRef;
use sp_core::redjubjub::Public;
use sp_core::{blake2_128, Encode};
use sp_io::hashing::twox_128;
use sp_runtime::codec::Compact;

pub fn encode_extrinsic<S: Encode, C: Encode>(signature: Option<S>, call: C) -> Vec<u8> {
    let mut tmp: Vec<u8> = vec![];

    const EXTRINSIC_VERSION: u8 = 4;
    match signature.as_ref() {
        Some(s) => {
            tmp.push(EXTRINSIC_VERSION | 0b1000_0000);
            s.encode_to(&mut tmp);
        }
        None => {
            tmp.push(EXTRINSIC_VERSION & 0b0111_1111);
        }
    }

    call.encode_to(&mut tmp);

    let compact_len = Compact(tmp.len() as u32);

    let mut output: Vec<u8> = vec![];
    compact_len.encode_to(&mut output);
    output.extend(tmp);

    output
}

pub(crate) fn encoded_key(module: &[u8], method: &[u8]) -> String {
    format!("{}{}", encode(twox_128(module)), encode(twox_128(method)))
}

pub(crate) fn black2_128concat(public_key: Public) -> String {
    let hash = blake2_128(public_key.as_bytes_ref());
    format!("{}{}", encode(hash), encode(public_key.0))
}
