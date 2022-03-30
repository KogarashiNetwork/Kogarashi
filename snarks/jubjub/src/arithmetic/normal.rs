use crate::arithmetic::utils::adc;

pub(crate) fn add(a: &mut [u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut c = 0;
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a = adc(*a, *b, &mut c)
    }
    *a
}

// pub(crate) fn sub(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
//     [r0, r1, r2, r3]
// }

pub(crate) fn double(a: &mut [u64; 4]) -> [u64; 4] {
    let mut c = 0;
    for a in a.iter_mut() {
        *a = adc(*a, *a, &mut c)
    }
    *a
}

// pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
//     [r0, r1, r2, r3]
// }

// pub(crate) fn square(a: &[u64; 4]) -> [u64; 4] {
//     [r0, r1, r2, r3]
// }

// pub(crate) fn neg(a: &[u64; 4]) -> [u64; 4] {
//     [r0, r1, r2, r3];
// }
