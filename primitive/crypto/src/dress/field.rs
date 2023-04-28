mod group;
mod ring;
mod test;

pub use group::*;
pub use ring::*;
pub use test::*;

#[macro_export]
macro_rules! prime_field_operation {
    ($field:ident, $p:ident, $g:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        field_operation!($field, $p, $g, $r, $inv, $r, $r2, $r3);

        impl ParityCmp for $field {}
        impl Basic for $field {}

        impl Debug for $field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for limb in self.montgomery_reduce().iter().rev() {
                    for byte in limb.to_be_bytes() {
                        write!(f, "{:02x}", byte)?;
                    }
                }
                Ok(())
            }
        }

        impl PrimeField for $field {
            const MODULUS: Self = $field($p);

            const INV: u64 = $inv;

            fn is_zero(self) -> bool {
                self.0.iter().all(|x| *x == 0)
            }

            fn to_bits(self) -> Bits {
                to_bits(self.montgomery_reduce())
            }

            fn to_nafs(self) -> Nafs {
                to_nafs(self.montgomery_reduce())
            }

            fn double(self) -> Self {
                Self(double(self.0, $p))
            }

            fn square(self) -> Self {
                Self(square(self.0, $p, $inv))
            }

            fn double_assign(&mut self) {
                self.0 = double(self.0, $p)
            }

            fn square_assign(&mut self) {
                self.0 = square(self.0, $p, $inv)
            }
        }
    };
}

#[macro_export]
macro_rules! fft_field_operation {
    ($field:ident, $p:ident, $g:ident, $mul_g:ident, $i:ident, $u:ident, $r:ident, $r2:ident, $r3:ident, $s:ident) => {
        prime_field_operation!($field, $p, $g, $i, $r, $r2, $r3);

        impl FftField for $field {
            const S: usize = $s;

            const ROOT_OF_UNITY: Self = $u;

            const MULTIPLICATIVE_GENERATOR: Self = $mul_g;

            fn pow(self, val: u64) -> Self {
                Self(pow(self.0, [val, 0, 0, 0], $r, $p, $i))
            }

            fn pow_of_2(by: u64) -> Self {
                let two = Self::from(2u64);
                let mut res = Self::one();
                for i in (0..64).rev() {
                    res = res.square();
                    let mut tmp = res;
                    tmp *= two;
                    res.conditional_assign(&tmp, (((by >> i) & 0x1) as u8).into());
                }
                res
            }

            fn divn(&mut self, mut n: u32) {
                if n >= 256 {
                    *self = Self::from(0u64);
                    return;
                }

                while n >= 64 {
                    let mut t = 0;
                    for i in self.0.iter_mut().rev() {
                        core::mem::swap(&mut t, i);
                    }
                    n -= 64;
                }

                if n > 0 {
                    let mut t = 0;
                    for i in self.0.iter_mut().rev() {
                        let t2 = *i << (64 - n);
                        *i >>= n;
                        *i |= t;
                        t = t2;
                    }
                }
            }

            fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
                Self(from_u512(
                    [
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[32..40]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[40..48]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[48..56]).unwrap()),
                        u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[56..64]).unwrap()),
                    ],
                    $r2,
                    $r3,
                    $p,
                    $i,
                ))
            }

            fn reduce(&self) -> Self {
                Self(mont(
                    [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
                    $p,
                    $i,
                ))
            }

            fn is_even(&self) -> bool {
                self.0[0] % 2 == 0
            }

            fn mod_2_pow_k(&self, k: u8) -> u8 {
                (self.0[0] & ((1 << k) - 1)) as u8
            }
            fn mod_by_window(&self, c: usize) -> u64 {
                self.0[0] % (1 << c)
            }

            fn mods_2_pow_k(&self, w: u8) -> i8 {
                assert!(w < 32u8);
                let modulus = self.mod_2_pow_k(w) as i8;
                let two_pow_w_minus_one = 1i8 << (w - 1);

                match modulus >= two_pow_w_minus_one {
                    false => modulus,
                    true => modulus - ((1u8 << w) as i8),
                }
            }
        }

        impl subtle::ConditionallySelectable for $field {
            fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
                $field([
                    u64::conditional_select(&a.0[0], &b.0[0], choice),
                    u64::conditional_select(&a.0[1], &b.0[1], choice),
                    u64::conditional_select(&a.0[2], &b.0[2], choice),
                    u64::conditional_select(&a.0[3], &b.0[3], choice),
                ])
            }
        }

        impl From<u64> for $field {
            fn from(val: u64) -> $field {
                $field(from_u64(val, $r2, $p, $i))
            }
        }

        impl From<[u64; 4]> for $field {
            fn from(val: [u64; 4]) -> $field {
                $field(val)
            }
        }

        impl ParallelCmp for $field {}

        impl RefOps for $field {}
    };
}

#[macro_export]
macro_rules! field_operation {
    ($field:ident, $p:ident, $g:ident, $e:ident, $inv:ident, $r:ident, $r2:ident, $r3:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        ring_operation!($field, $p, $g, $e, $r2, $r3, $inv);

        impl Field for $field {}

        impl Div for $field {
            type Output = $field;

            #[inline]
            fn div(self, rhs: $field) -> $field {
                let inv = rhs.invert().unwrap();
                self * inv
            }
        }

        impl DivAssign for $field {
            fn div_assign(&mut self, rhs: $field) {
                let inv = rhs.invert().unwrap();
                *self *= inv
            }
        }
    };
}

pub use {fft_field_operation, field_operation, prime_field_operation};
