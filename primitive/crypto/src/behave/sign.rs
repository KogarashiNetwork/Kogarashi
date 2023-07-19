pub trait SigUtils: Sized {
    const LENGTH: usize;

    fn to_bytes(self) -> [u8; 32];

    fn from_bytes(bytes: [u8; 32]) -> Option<Self>;
}
