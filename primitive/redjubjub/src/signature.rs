use zkstd::behave::SigUtils;

#[derive(Clone)]
pub struct Signature {
    pub(crate) r: [u8; 32],
    pub(crate) s: [u8; 32],
}

impl SigUtils<64> for Signature {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&bytes[..32]);
        s.copy_from_slice(&bytes[32..]);
        Some(Self { r, s })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&self.r);
        bytes[32..].copy_from_slice(&self.s);
        bytes
    }
}

impl Signature {
    pub(crate) fn new(r: [u8; 32], s: [u8; 32]) -> Self {
        Self { r, s }
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), Self::LENGTH);
        let bytes: [u8; Self::LENGTH] = bytes[..64].try_into().unwrap();
        Self::from_bytes(bytes)
    }
}
