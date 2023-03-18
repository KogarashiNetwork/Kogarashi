pub trait SignatureRepr {
    // convert little-endian 32-octets string
    fn encode_for_sig(self) -> [u8; 32];

    // decode element from 32-octets string
    fn decode_for_sig(raw: [u8; 32]) -> Self;
}
