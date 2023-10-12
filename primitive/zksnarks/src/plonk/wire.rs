#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PrivateWire(usize);

impl PrivateWire {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}
