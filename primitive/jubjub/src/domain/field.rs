macro_rules! field_operation {
    ($field:ident, $p:ident) => {
        // basic arithmetic
        impl $field {
            #[inline(always)]
            pub fn double_assign(&mut self) {
                self.0 .0 = double(&self.0 .0, &$p.0)
            }

            #[inline(always)]
            pub fn square_assign(&mut self) {
                self.0 .0 = square(&self.0 .0, &$p.0)
            }

            pub const fn zero() -> $field {
                $field(FrRaw([0, 0, 0, 0]))
            }

            pub fn one() -> $field {
                $field::from_raw(FrRaw([1, 0, 0, 0])).unwrap()
            }
        }

        impl Add for $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: $field) -> $field {
                $field(FrRaw(add(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl<'a, 'b> Add<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn add(self, rhs: &'b $field) -> $field {
                $field(FrRaw(add(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl AddAssign for $field {
            fn add_assign(&mut self, rhs: $field) {
                self.0 .0 = add(&self.0 .0, &rhs.0 .0, &$p.0)
            }
        }

        impl Sub for $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: $field) -> $field {
                $field(FrRaw(sub(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl<'a, 'b> Sub<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn sub(self, rhs: &'b $field) -> $field {
                $field(FrRaw(sub(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl SubAssign for $field {
            fn sub_assign(&mut self, rhs: $field) {
                self.0 .0 = sub(&self.0 .0, &rhs.0 .0, &$p.0)
            }
        }

        impl Mul for $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: $field) -> $field {
                $field(FrRaw(mul(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $field {
            type Output = $field;

            #[inline]
            fn mul(self, rhs: &'b $field) -> $field {
                $field(FrRaw(mul(&self.0 .0, &rhs.0 .0, &$p.0)))
            }
        }

        impl MulAssign for $field {
            fn mul_assign(&mut self, rhs: $field) {
                self.0 .0 = mul(&self.0 .0, &rhs.0 .0, &$p.0)
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
                self.0.is_zero()
            }

            pub fn double(self) -> $field {
                $field(FrRaw(double(&self.0 .0, &$p.0)))
            }

            pub fn square(self) -> $field {
                $field(FrRaw(square(&self.0 .0, &$p.0)))
            }

            pub fn invert(self) -> Option<$field> {
                Self::_invert(&self).map(|x| $field(FrRaw(x)))
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
                    false => Some(t0.0 .0),
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
                $field(FrRaw(neg(&self.0 .0, &$p.0)))
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
                write!(f, "Fs({})", self.into_raw())
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
