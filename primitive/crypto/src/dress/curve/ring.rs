#[macro_export]
macro_rules! projective_ring_operation {
    ($projective:ident, $field:ident, $g:ident, $e:ident) => {
        projective_group_operation!($projective, $g, $e);

        impl Ring for $projective {}

        impl Add for $projective {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $projective) -> Self {
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, rhs.y, rhs.z));
                Self { x, y, z }
            }
        }

        impl<'a, 'b> Add<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn add(self, rhs: &'b $projective) -> $projective {
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, rhs.y, rhs.z));
                Self { x, y, z }
            }
        }

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, rhs.y, rhs.z));
                self.0 = Self { x, y, z }
            }
        }

        impl Mul for $projective {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $field) -> Self {
                let mut res = Projective::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .as_bits()
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

        impl<'a, 'b> Mul<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn mul(self, scalar: &'b $field) -> $projective {
                let mut res = Projective::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .as_bits()
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

        impl MulAssign for $projective {
            fn mul_assign(&mut self, scalar: $field) {
                let mut res = Projective::IDENTITY;
                let mut acc = self.clone();
                let bits: Vec<u8> = scalar
                    .as_bits()
                    .into_iter()
                    .skip_while(|x| *x == 0)
                    .collect();
                for &b in bits.iter().rev() {
                    if b == 1 {
                        res += acc.clone();
                    }
                    acc.double();
                }
                self = res
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
                Self {
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
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, -rhs.y, rhs.z));
                Self { x, y, z }
            }
        }

        impl<'a, 'b> Sub<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn sub(self, rhs: &'b $projective) -> $projective {
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, -rhs.y, rhs.z));
                Self { x, y, z }
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                let (x, y, z) = add_point((self.x, self.y, self.z), (rhs.x, -rhs.y, rhs.z));
                self.0 = Self { x, y, z }
            }
        }
    };
}

pub use projective_ring_operation;
