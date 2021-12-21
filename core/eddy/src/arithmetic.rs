use crate::fr::MODULUS;

pub(crate) fn add(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
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

            m_ptr = in(reg) MODULUS.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

pub(crate) fn sub(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
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

            // sub a array and b array with borrow
            "sub r8, qword ptr [{b_ptr} + 0]",
            "sbb r9, qword ptr [{b_ptr} + 8]",
            "sbb r10, qword ptr [{b_ptr} + 16]",
            "sbb r11, qword ptr [{b_ptr} + 24]",

            // copy result array to latter registers
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod addition
            "add r12, qword ptr [{m_ptr} + 0]",
            "adc r13, qword ptr [{m_ptr} + 8]",
            "adc r14, qword ptr [{m_ptr} + 16]",
            "adc r15, qword ptr [{m_ptr} + 24]",

            // if not carry copy former registers to out areas
            "cmovnc r12, r8",
            "cmovnc r13, r9",
            "cmovnc r14, r10",
            "cmovnc r15, r11",

            m_ptr = in(reg) MODULUS.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

pub(crate) fn double(a: &[u64; 4]) -> [u64; 4] {
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

            m_ptr = in(reg) MODULUS.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}
