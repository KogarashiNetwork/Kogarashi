#[derive(Clone)]
pub struct Signature {
    pub(crate) r: [u8; 32],
    pub(crate) s: [u8; 32],
}

impl Signature {
    pub(crate) fn new(r: [u8; 32], s: [u8; 32]) -> Self {
        Self { r, s }
    }
}
