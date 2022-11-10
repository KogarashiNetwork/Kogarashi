use core::arch::asm;

pub(crate) fn add(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // add a array and b array with carry
            "add r8, qword ptr [{b_ptr} + 0]",
            "adc r9, qword ptr [{b_ptr} + 8]",
            "adc r10, qword ptr [{b_ptr} + 16]",
            "adc r11, qword ptr [{b_ptr} + 24]",

            // copy result array to latter registers
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod reduction
            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            // if carry copy former registers to out areas
            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) p.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

pub(crate) fn sub(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            // init modulus area
            "xor r12, r12",
            "xor r13, r13",
            "xor r14, r14",
            "xor r15, r15",

            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // sub a array and b array with borrow
            "sub r8, qword ptr [{b_ptr} + 0]",
            "sbb r9, qword ptr [{b_ptr} + 8]",
            "sbb r10, qword ptr [{b_ptr} + 16]",
            "sbb r11, qword ptr [{b_ptr} + 24]",

            // if carry copy modulus
            "cmovc r12, qword ptr [{m_ptr} + 0]",
            "cmovc r13, qword ptr [{m_ptr} + 8]",
            "cmovc r14, qword ptr [{m_ptr} + 16]",
            "cmovc r15, qword ptr [{m_ptr} + 24]",

            // mod addition
            "add  r12, r8",
            "adc  r13, r9",
            "adc  r14, r10",
            "adc  r15, r11",

            m_ptr = in(reg) p.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

pub(crate) fn double(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // // add a array and b array with carry
            "add r8, r8",
            "adc r9, r9",
            "adc r10, r10",
            "adc r11, r11",

            // copy result array to latter registers
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod reduction
            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            // if carry copy former registers to out areas
            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) p.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

pub(crate) fn mul(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            // schoolbook multiplication
            //    *    |   a0    |   a1    |   a2    |   a3
            //    b0   | b0 * a0 | b0 * a1 | b0 * a2 | b0 * a3
            //    b1   | b1 * a0 | b1 * a1 | b1 * a2 | b1 * a3
            //    b2   | b2 * a0 | b2 * a1 | b2 * a2 | b2 * a3
            //    b3   | b3 * a0 | b3 * a1 | b3 * a2 | b3 * a3

            // quotient
            //    r8   | a0 * b0 |         |         |
            //    r9   | a0 * b1 | a1 * b0 |         |
            //    r10  | a0 * b2 | a1 * b1 | a2 * b0 |
            //    r11  | a0 * b3 | a1 * b2 | a2 * b1 | a3 * b0 |
            //    r12  | a1 * b3 | a2 * b2 | a3 * b1 |
            //    r13  | a2 * b3 | a3 * b2 |         |
            //    r14  | a3 * b3 |         |         |

            // carry
            //    r9   | 00  |     |     |
            //    r10  | 01  | 10  |     |
            //    r11  | 02  | 11  | 20  |
            //    r12  | 03  | 12  | 21  | 30
            //    r13  | 13  | 22  | 31  |
            //    r14  | 23  | 32  |     |
            //    r15  | 33  |     |     |

            "mov r13, qword ptr [{b_ptr} + 0]",
            "mov r14, qword ptr [{b_ptr} + 8]",
            "mov r15, qword ptr [{b_ptr} + 16]",

            // `a0`
            "mov rdx, qword ptr [{a_ptr} + 0]",

            // a0 * b0
            "mulx r9, r8, r13",

            // a0 * b1
            "mulx r10, rax, r14",
            "add r9, rax",

            // a0 * b2
            "mulx r11, rax, r15",
            "adcx r10, rax",

            // a0 * b3
            "mulx r12, rax, qword ptr [{b_ptr} + 24]",
            "adcx r11, rax",
            "adc r12, 0",

            // `a1`
            "mov rdx, [{a_ptr} + 8]",

            // a1 * b0
            "mulx rcx, rax, r13",
            "add r9, rax",
            "adcx r10, rcx",
            "adc r11, 0",

            // a1 * b1
            "mulx rcx, rax, r14",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",
            "xor r13, r13",

            // a1 * b2
            "mulx rcx, rax, r15",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",
            "xor r14, r14",

            // a1 * b3
            "mulx rcx, rax, qword ptr [{b_ptr} + 24]",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",

            // `a2`
            "mov rdx, [{a_ptr} + 16]",

            // a2 * b0
            "mulx rcx, rax, qword ptr [{b_ptr} + 0]",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",

            // a2 * b1
            "mulx rcx, rax, qword ptr [{b_ptr} + 8]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // a2 * b2
            "mulx rcx, rax, r15",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",
            "xor r15, r15",

            // a2 * b3
            "mulx rcx, rax, qword ptr [{b_ptr} + 24]",
            "add r13, rax",
            "adcx r14, rcx",
            "adc r15, 0",

            // `a3`
            "mov rdx, [{a_ptr} + 24]",

            // a3 * b0
            "mulx rcx, rax, qword ptr [{b_ptr} + 0]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // a3 * b1
            "mulx rcx, rax, qword ptr [{b_ptr} + 8]",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",

            // a3 * b2
            "mulx rcx, rax, qword ptr [{b_ptr} + 16]",
            "add r13, rax",
            "adcx r14, rcx",
            "adc r15, 0",

            // a3 * b3
            "mulx rcx, rax, qword ptr [{b_ptr} + 24]",
            "add r14, rax",
            "adc r15, rcx",

            // montgomery reduction
            // r8 ~ r15

            // `r8` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r8",

            // r8' * m0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "add r8, rax",
            "adcx r9, rcx",
            "adc r10, 0",

            // r8' * m1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "add r9, rax",
            "adcx r10, rcx",
            "adc r11, 0",

            // // r8' * m2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",

            // // r8' * m3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // `r9` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r9",

            // r9' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r9, rcx",
            "adcx r10, rax",
            "adc r11, 0",

            // r9' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r10, rcx",
            "adcx r11, rax",
            "adc r12, 0",

            // r9' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r9' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // `r10` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r10",

            // r10' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r10, rcx",
            "adcx r11, rax",
            "adc r12, 0",

            // r10' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r10' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // r10' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r13, rcx",
            "adcx r14, rax",
            "adc r15, 0",

            // `r11` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r11",

            // r11' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r11' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // r11' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r13, rcx",
            "adcx r14, rax",
            "adc r15, 0",

            // r11' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r14, rcx",
            "adcx r15, rax",

            // reduction if limbs is greater then mod
            "mov r8, r12",
            "mov r9, r13",
            "mov r10, r14",
            "mov r11, r15",

            "sub r8, qword ptr [{m_ptr} + 0]",
            "sbb r9, qword ptr [{m_ptr} + 8]",
            "sbb r10, qword ptr [{m_ptr} + 16]",
            "sbb r11, qword ptr [{m_ptr} + 24]",

            "cmovc r8, r12",
            "cmovc r9, r13",
            "cmovc r10, r14",
            "cmovc r11, r15",

            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) p.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("rax") _,
            out("rcx") _,
            out("rdx") _,
            out("r8") r0,
            out("r9") r1,
            out("r10") r2,
            out("r11") r3,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            options(pure, readonly, nostack)
        )
    }
    [r0, r1, r2, r3]
}

pub(crate) fn square(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            "mov r12, qword ptr [{a_ptr} + 0]",
            "mov r13, qword ptr [{a_ptr} + 8]",
            "mov r14, qword ptr [{a_ptr} + 16]",
            "mov r15, qword ptr [{a_ptr} + 24]",

            // `a0`
            "mov rdx, r12",

            // a0 * b0
            "mulx r9, r8, r12",

            // a0 * b1
            "mulx r10, rax, r13",
            "add r9, rax",
            "adc r10, 0",

            // a0 * b2
            "mulx r11, rax, r14",
            "add r10, rax",
            "adc r11, 0",

            // a0 * b3
            "mulx r12, rax, r15",
            "adcx r11, rax",
            "adc r12, 0",

            // `a1`
            "mov rdx, r13",

            // a1 * b0
            "mulx rcx, rax, qword ptr [{a_ptr} + 0]",
            "add r9, rax",
            "adcx r10, rcx",
            "adc r11, 0",

            // a1 * b1
            "mulx rcx, rax, r13",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",
            "xor r13, r13",

            // a1 * b2
            "mulx rcx, rax, r14",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",
            "xor r14, r14",

            // a1 * b3
            "mulx rcx, rax, r15",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",
            "xor r15, r15",

            // `a2`
            "mov rdx, [{a_ptr} + 16]",

            // a2 * b0
            "mulx rcx, rax, qword ptr [{a_ptr} + 0]",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",

            // a2 * b1
            "mulx rcx, rax, qword ptr [{a_ptr} + 8]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // a2 * b2
            "mulx rcx, rax, qword ptr [{a_ptr} + 16]",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",

            // a2 * b3
            "mulx rcx, rax, qword ptr [{a_ptr} + 24]",
            "adcx r13, rax",
            "adcx r14, rcx",
            "adc r15, 0",

            // `a3`
            "mov rdx, [{a_ptr} + 24]",

            // a3 * b0
            "mulx rcx, rax, qword ptr [{a_ptr} + 0]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // a3 * b1
            "mulx rcx, rax, qword ptr [{a_ptr} + 8]",
            "add r12, rax",
            "adcx r13, rcx",
            "adc r14, 0",

            // a3 * b2
            "mulx rcx, rax, qword ptr [{a_ptr} + 16]",
            "add r13, rax",
            "adcx r14, rcx",
            "adc r15, 0",

            // a3 * b3
            "mulx rcx, rax, qword ptr [{a_ptr} + 24]",
            "add r14, rax",
            "adc r15, rcx",

            // montgomery reduction
            // r8 ~ r15

            // `r8` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r8",

            // r8' * m0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "add r8, rax",
            "adcx r9, rcx",
            "adc r10, 0",

            // r8' * m1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "add r9, rax",
            "adcx r10, rcx",
            "adc r11, 0",

            // // r8' * m2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "add r10, rax",
            "adcx r11, rcx",
            "adc r12, 0",

            // // r8' * m3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "add r11, rax",
            "adcx r12, rcx",
            "adc r13, 0",

            // `r9` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r9",

            // r9' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r9, rcx",
            "adcx r10, rax",
            "adc r11, 0",

            // r9' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r10, rcx",
            "adcx r11, rax",
            "adc r12, 0",

            // r9' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r9' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // `r10` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r10",

            // r10' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r10, rcx",
            "adcx r11, rax",
            "adc r12, 0",

            // r10' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r10' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // r10' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r13, rcx",
            "adcx r14, rax",
            "adc r15, 0",

            // `r11` -> 0
            "mov rdx, 0x1ba3a358ef788ef9",
            "mulx rax, rdx, r11",

            // r11' * m0
            "mulx rax, rcx, qword ptr [{m_ptr} + 0]",
            "add r11, rcx",
            "adcx r12, rax",
            "adc r13, 0",

            // r11' * m1
            "mulx rax, rcx, qword ptr [{m_ptr} + 8]",
            "add r12, rcx",
            "adcx r13, rax",
            "adc r14, 0",

            // r11' * m2
            "mulx rax, rcx, qword ptr [{m_ptr} + 16]",
            "add r13, rcx",
            "adcx r14, rax",
            "adc r15, 0",

            // r11' * m3
            "mulx rax, rcx, qword ptr [{m_ptr} + 24]",
            "add r14, rcx",
            "adcx r15, rax",

            // reduction if limbs is greater then mod
            "mov r8, r12",
            "mov r9, r13",
            "mov r10, r14",
            "mov r11, r15",

            "sub r8, qword ptr [{m_ptr} + 0]",
            "sbb r9, qword ptr [{m_ptr} + 8]",
            "sbb r10, qword ptr [{m_ptr} + 16]",
            "sbb r11, qword ptr [{m_ptr} + 24]",

            "cmovc r8, r12",
            "cmovc r9, r13",
            "cmovc r10, r14",
            "cmovc r11, r15",

            m_ptr = in(reg) p.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            out("rax") _,
            out("rcx") _,
            out("rdx") _,
            out("r8") r0,
            out("r9") r1,
            out("r10") r2,
            out("r11") r3,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            options(pure, readonly, nostack)
        )
    }
    [r0, r1, r2, r3]
}

pub(crate) fn neg(a: &[u64; 4], p: &[u64; 4]) -> [u64; 4] {
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    unsafe {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{m_ptr} + 0]",
            "mov r9, qword ptr [{m_ptr} + 8]",
            "mov r10, qword ptr [{m_ptr} + 16]",
            "mov r11, qword ptr [{m_ptr} + 24]",

            "sub r8, qword ptr [{a_ptr} + 0]",
            "sbb r9, qword ptr [{a_ptr} + 8]",
            "sbb r10, qword ptr [{a_ptr} + 16]",
            "sbb r11, qword ptr [{a_ptr} + 24]",

            "mov r12, qword ptr [{a_ptr} + 0]",
            "mov r13, qword ptr [{a_ptr} + 8]",
            "mov r14, qword ptr [{a_ptr} + 16]",
            "mov r15, qword ptr [{a_ptr} + 24]",

            "or r12, r13",
            "or r14, r15",
            "or r12, r14",

            "mov r13, 0xffffffffffffffff",
            "cmp r12, 0x0000000000000000",
            "cmove r13, r12",

            "and r8, r13",
            "and r9, r13",
            "and r10, r13",
            "and r11, r13",

            a_ptr = in(reg) a.as_ptr(),
            m_ptr = in(reg) p.as_ptr(),
            out("r8") r0,
            out("r9") r1,
            out("r10") r2,
            out("r11") r3,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            options(pure, readonly, nostack)
        )
    }
    [r0, r1, r2, r3]
}
