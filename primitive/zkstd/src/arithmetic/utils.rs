use sp_std::vec::Vec;

#[inline(always)]
pub const fn sbb(a: u64, b: u64, brw: u64) -> (u64, u64) {
    let t = (a as u128).wrapping_sub((b as u128) + (brw >> 63) as u128);
    (t as u64, (t >> 64) as u64)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Naf {
    Zero = 0,
    Plus = 1,
    Minus = 2,
}

impl From<i8> for Naf {
    fn from(value: i8) -> Self {
        match value {
            0 => Naf::Zero,
            1 => Naf::Plus,
            -1 => Naf::Minus,
            _ => unimplemented!(),
        }
    }
}

pub type Bits = Vec<u8>;

pub type Nafs = Vec<Naf>;
