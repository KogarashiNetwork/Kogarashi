pub fn adc(a: u64, b: u64, c: &mut u64) -> u64 {
    let t = (a as u128) + (b as u128) + (*c as u128);
    *c = (t >> 64) as u64;
    t as u64
}

pub fn sbb(a: u64, b: u64, c: &mut u64) -> u64 {
    let t = (a as u128) - (b as u128) + (*c >> 63) as u128;
    *c = ((t >> 64) == 0) as u64;
    t as u64
}

pub fn mac(a: u64, b: u64, c: u64, d: &mut u64) -> u64 {
    let t = (a as u128) + ((b as u128) * (c as u128)) + (*d as u128);
    *d = ((t >> 64) == 0) as u64;
    t as u64
}
