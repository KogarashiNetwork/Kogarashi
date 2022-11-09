#[macro_export]
macro_rules! projective_ring_operation {
    ($projective:ident, $field:ident, $g:ident, $e:ident) => {
        projective_group_operation!($projective, $g, $e);

        impl Ring for $projective {}

        impl Add for $projective {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $projective) -> Self {
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, rhs.y.0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                Self {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                }
            }
        }

        impl<'a, 'b> Add<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn add(self, rhs: &'b $projective) -> $projective {
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, rhs.y.0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                $projective {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                }
            }
        }

        impl AddAssign for $projective {
            fn add_assign(&mut self, rhs: $projective) {
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, rhs.y.0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                let mut res = $projective {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                };
                self = &mut res
            }
        }

        impl Mul<$field> for $projective {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $field) -> Self {
                let mut res = Self::Output::IDENTITY;
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

        impl<'a, 'b> Mul<&'b $field> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn mul(self, scalar: &'b $field) -> $projective {
                let mut res = Self::Output::IDENTITY;
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

        impl MulAssign<$field> for $projective {
            fn mul_assign(&mut self, scalar: $field) {
                let mut res = Self::IDENTITY;
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
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, (-rhs.y).0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                Self {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                }
            }
        }

        impl<'a, 'b> Sub<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn sub(self, rhs: &'b $projective) -> $projective {
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, (-rhs.y).0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                $projective {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                }
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                let (x, y, z) = add_point(
                    (self.x.0, self.y.0, self.z.0),
                    (rhs.x.0, (-rhs.y).0, rhs.z.0),
                    $field::MODULUS.0,
                    $field::INV,
                );
                let mut res = $projective {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                };
                self = &mut res
            }
        }
    };
}

pub use projective_ring_operation;
