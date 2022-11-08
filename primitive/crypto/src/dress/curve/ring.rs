#[macro_export]
macro_rules! projective_ring_operation {
    ($projective:ident, $g:ident, $e:ident) => {
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
            fn mul(self, rhs: $projective) -> Self {
                $projective(mul(self.0, rhs.0, $p.0, $inv))
            }
        }

        impl<'a, 'b> Mul<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn mul(self, rhs: &'b $projective) -> $projective {
                $projective(mul(self.0, rhs.0, $p.0, $inv))
            }
        }

        impl MulAssign for $projective {
            fn mul_assign(&mut self, rhs: $projective) {
                self.0 = mul(self.0, rhs.0, $p.0, $inv)
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
                $projective(sub(self.0, rhs.0, $p.0))
            }
        }

        impl<'a, 'b> Sub<&'b $projective> for &'a $projective {
            type Output = $projective;

            #[inline]
            fn sub(self, rhs: &'b $projective) -> $projective {
                $projective(sub(self.0, rhs.0, $p.0))
            }
        }

        impl SubAssign for $projective {
            fn sub_assign(&mut self, rhs: $projective) {
                self.0 = sub(self.0, rhs.0, $p.0)
            }
        }
    };
}

pub use projective_ring_operation;
