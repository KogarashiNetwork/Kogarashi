#[macro_export]
macro_rules! projective_ring_operation {
    ($projective:ident, $field:ident, $g:ident, $e:ident) => {
        projective_group_operation!($projective, $g, $e);

        impl Ring for $projective {}

        impl Add for $projective {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $projective) -> Self {
                add_point(self, rhs)
            }
        }

        impl<'a, 'b> Add<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn add(self, rhs: &'b $projective) -> $projective {
                add_point(self.clone(), rhs.clone())
            }
        }

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                *self = self.add(rhs);
            }
        }

        impl Mul<$field> for $projective {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $field) -> Self {
                let mut res = Self::Output::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .to_bits()
                    .into_iter()
                    .skip_while(|x| *x == 0)
                    .collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc = acc.double();
                }
                res
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn mul(self, scalar: &'b $field) -> $projective {
                let mut res = Self::Output::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .to_bits()
                    .into_iter()
                    .skip_while(|x| *x == 0)
                    .collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc.double();
                }
                res
            }
        }

        impl MulAssign<$field> for $projective {
            fn mul_assign(&mut self, scalar: $field) {
                let mut res = Self::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .to_bits()
                    .into_iter()
                    .skip_while(|x| *x == 0)
                    .collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc.double();
                }
                *self = res
            }
        }

        impl Neg for $projective {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    z: self.z,
                }
            }
        }

        impl<'a> Neg for &'a $projective {
            type Output = $projective;

            #[inline]
            fn neg(self) -> $projective {
                $projective {
                    x: self.x,
                    y: -self.y,
                    z: self.z,
                }
            }
        }

        impl Sub for $projective {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $projective) -> Self {
                add_point(self, rhs.neg())
            }
        }

        impl<'a, 'b> Sub<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn sub(self, rhs: &'b $projective) -> $projective {
                add_point(self.clone(), rhs.neg())
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                *self = self.add(rhs.neg());
            }
        }
    };
}

pub use projective_ring_operation;
