macro_rules! field_operation {
    ($field:ident, $p:ident) => {
        // basic arithmetic
        impl $field {
            #[inline(always)]
            pub fn double_assign(&mut self) {
                self.0 = double(&self.0, $p)
            }

            #[inline(always)]
            pub fn square_assign(&mut self) {
                self.0 = square(&self.0, $p)
            }

            pub const fn zero() -> $field {
                $field([0, 0, 0, 0])
            }

            pub fn one() -> $field {
                $field::from_raw([1, 0, 0, 0])
            }
        }

        impl Add for $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: $field) -> $field {
                $field(add(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                $field(add(&self.0, &rhs.0, $p))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 = add(&self.0, &rhs.0, $p)
            }
        }

        impl Sub for $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: $field) -> $field {
                $field(sub(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                $field(sub(&self.0, &rhs.0, $p))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 = sub(&self.0, &rhs.0, $p)
            }
        }

        impl Mul for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: $field) -> $field {
                $field(mul(&self.0, &rhs.0, $p))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                $field(mul(&self.0, &rhs.0, $p))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 = mul(&self.0, &rhs.0, $p)
            }
        }

        impl Mul<Projective> for $field {
            type Output = Projective;
            fn mul(self, rhs: Projective) -> Self::Output {
                let mut res = Projective::identity();
                let mut acc = rhs.clone();
                // TODO
                let bits: Vec<u8> = self.as_bits().into_iter().skip_while(|x| *x == 0).collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc.double();
                }
                res
            }
        }

        impl Mul<$field> for Projective {
            type Output = Projective;
            fn mul(self, rhs: $field) -> Self::Output {
                let mut res = Projective::identity();
                let mut acc = self.clone();
                // TODO
                let bits: Vec<u8> = rhs.as_bits().into_iter().skip_while(|x| *x == 0).collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc.double();
                }
                res
            }
        }

        impl $field {
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|x| *x == 0)
            }

            pub fn double(self) -> $field {
                $field(double(&self.0, $p))
            }

            pub fn square(self) -> $field {
                $field(square(&self.0, $p))
            }

            pub fn invert(self) -> Option<$field> {
                Self::_invert(&self).map(|x| $field(x))
            }

            pub fn random(mut rand: impl RngCore) -> $field {
                Fr::from_u512([
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                    rand.next_u64(),
                ])
            }

            #[inline]
            fn _invert(x: &$field) -> Option<[u64; 4]> {
                #[inline(always)]
                fn square_assign_multi(n: &mut Fr, num_times: usize) {
                    for _ in 0..num_times {
                        *n = n.square();
                    }
                }
                let mut t1 = x.square();
                let mut t0 = t1.square();
                let mut t3 = t0 * t1;
                let t6 = t3 * *x;
                let t7 = t6 * t1;
                let t12 = t7 * t3;
                let t13 = t12 * t0;
                let t16 = t12 * t3;
                let t2 = t13 * t3;
                let t15 = t16 * t3;
                let t19 = t2 * t0;
                let t9 = t15 * t3;
                let t18 = t9 * t3;
                let t14 = t18 * t1;
                let t4 = t18 * t0;
                let t8 = t18 * t3;
                let t17 = t14 * t3;
                let t11 = t8 * t3;
                t1 = t17 * t3;
                let t5 = t11 * t3;
                t3 = t5 * t0;
                t0 = t5.square();
                square_assign_multi(&mut t0, 5);
                t0 *= t3;
                square_assign_multi(&mut t0, 6);
                t0 *= t8;
                square_assign_multi(&mut t0, 7);
                t0 *= t19;
                square_assign_multi(&mut t0, 6);
                t0 *= t13;
                square_assign_multi(&mut t0, 8);
                t0 *= t14;
                square_assign_multi(&mut t0, 6);
                t0 *= t18;
                square_assign_multi(&mut t0, 7);
                t0 *= t17;
                square_assign_multi(&mut t0, 5);
                t0 *= t16;
                square_assign_multi(&mut t0, 3);
                t0 *= *x;
                square_assign_multi(&mut t0, 11);
                t0 *= t11;
                square_assign_multi(&mut t0, 8);
                t0 *= t5;
                square_assign_multi(&mut t0, 5);
                t0 *= t15;
                square_assign_multi(&mut t0, 8);
                t0 *= *x;
                square_assign_multi(&mut t0, 12);
                t0 *= t13;
                square_assign_multi(&mut t0, 7);
                t0 *= t9;
                square_assign_multi(&mut t0, 5);
                t0 *= t15;
                square_assign_multi(&mut t0, 14);
                t0 *= t14;
                square_assign_multi(&mut t0, 5);
                t0 *= t13;
                square_assign_multi(&mut t0, 2);
                t0 *= *x;
                square_assign_multi(&mut t0, 6);
                t0 *= *x;
                square_assign_multi(&mut t0, 9);
                t0 *= t7;
                square_assign_multi(&mut t0, 6);
                t0 *= t12;
                square_assign_multi(&mut t0, 8);
                t0 *= t11;
                square_assign_multi(&mut t0, 3);
                t0 *= *x;
                square_assign_multi(&mut t0, 12);
                t0 *= t9;
                square_assign_multi(&mut t0, 11);
                t0 *= t8;
                square_assign_multi(&mut t0, 8);
                t0 *= t7;
                square_assign_multi(&mut t0, 4);
                t0 *= t6;
                square_assign_multi(&mut t0, 10);
                t0 *= t5;
                square_assign_multi(&mut t0, 7);
                t0 *= t3;
                square_assign_multi(&mut t0, 6);
                t0 *= t4;
                square_assign_multi(&mut t0, 7);
                t0 *= t3;
                square_assign_multi(&mut t0, 5);
                t0 *= t2;
                square_assign_multi(&mut t0, 6);
                t0 *= t2;
                square_assign_multi(&mut t0, 7);
                t0 *= t1;

                match x.is_zero() {
                    true => None,
                    false => Some(t0.0),
                }
            }
        }

        impl Neg for $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
                -&self
            }
        }

        impl<'a> Neg for &'a $field {
            type Output = $field;

            #[inline]
            fn neg(self) -> $field {
                $field(neg(&self.0, $p))
            }
        }

        // comparison operation
        impl Eq for Fr {}

        impl PartialEq for Fr {
            fn eq(&self, other: &Self) -> bool {
                self.0[0] == other.0[0]
                    && self.0[1] == other.0[1]
                    && self.0[2] == other.0[2]
                    && self.0[3] == other.0[3]
            }
        }

        impl PartialOrd for $field {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }

            fn lt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                false
            }

            fn le(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a < b;
                    }
                }
                true
            }

            fn gt(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                false
            }

            fn ge(&self, other: &Self) -> bool {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a != b {
                        return a > b;
                    }
                }
                true
            }
        }

        impl Ord for Fr {
            fn cmp(&self, other: &Self) -> Ordering {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    if a < b {
                        return Ordering::Less;
                    } else if a > b {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
        }

        // basic trait
        impl Default for $field {
            fn default() -> Self {
                $field::zero()
            }
        }

        impl Display for $field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "0x")?;
                for i in self.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }

        impl Binary for $field {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                for b in self.as_bits().into_iter().skip_while(|x| *x == 0) {
                    write!(f, "{}", b)?;
                }
                Ok(())
            }
        }
    };
}

pub(crate) use field_operation;
