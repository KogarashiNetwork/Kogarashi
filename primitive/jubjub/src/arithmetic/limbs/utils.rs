pub(crate) const fn adc(a: u64, b: u64, c: u64) -> (u64, u64) {
    let t = a as u128 + b as u128 + c as u128;
    (t as u64, (t >> 64) as u64)
}

pub(crate) const fn rdc(a: u64, b: u64, c: u64) -> (u64, u64) {
    let t = a as i128 - b as i128 - c as i128;
    (t as u64, t.is_negative() as u64)
}

pub(crate) const fn sbb(a: u64, b: u64, c: u64) -> (u64, u64) {
    let t = a as i128 - b as i128 - c as i128;
    (t as u64, t.is_negative() as u64)
}

pub(crate) const fn mac(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
    let t = (a as u128) + ((b as u128) * (c as u128)) + (d as u128);
    (t as u64, (t >> 64) as u64)
}
