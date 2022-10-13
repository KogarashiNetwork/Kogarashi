use sp_core::Encode;
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
