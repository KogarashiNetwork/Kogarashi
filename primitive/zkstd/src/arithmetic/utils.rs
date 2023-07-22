use sp_std::vec::Vec;

#[inline(always)]
pub const fn adc(a: u64, b: u64, c: u64) -> (u64, u64) {
    let t = a as u128 + b as u128 + c as u128;
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub(crate) const fn addnc(a: u64, b: u64) -> (u64, u64) {
    let t = a as u128 + b as u128;
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub(crate) const fn mulnc(a: u64, b: u64) -> (u64, u64) {
    let t = a as u128 * b as u128;
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub const fn sbb(a: u64, b: u64, brw: u64) -> (u64, u64) {
    let t = (a as u128).wrapping_sub((b as u128) + (brw >> 63) as u128);
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub const fn mac(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
    let t = (a as u128) + ((b as u128) * (c as u128)) + (d as u128);
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub(crate) const fn muladd(a: u64, b: u64, c: u64) -> (u64, u64) {
    let t = a as u128 * b as u128 + c as u128;
    (t as u64, (t >> 64) as u64)
}

#[inline(always)]
pub(crate) const fn dbc(a: u64, b: u64) -> (u64, u64) {
    let a = a as u128;
    let t = (a << 1) + b as u128;
    (t as u64, (t >> 64) as u64)
}

pub type ProjectiveCoordinate<L> = (L, L, L);

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
