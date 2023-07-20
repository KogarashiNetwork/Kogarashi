pub trait SigUtils<const L: usize>: Sized {
    const LENGTH: usize = L;

    fn to_bytes(self) -> [u8; L];

    fn from_bytes(bytes: [u8; L]) -> Option<Self>;
}
