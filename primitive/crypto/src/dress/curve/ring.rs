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
                *self = *self * scalar;
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

#[macro_export]
macro_rules! affine_ring_operation {
    ($affine:ident, $field:ident, $g:ident, $e:ident) => {
        affine_group_operation!($affine, $g, $e);

        impl Ring for $affine {}

        impl Add for $affine {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $affine) -> Self {
                $affine::from(self.to_projective() + rhs.to_projective())
            }
        }

        impl<'a, 'b> Add<&'b $affine> for &'a $affine {
            type Output = $affine;

            #[inline]
            fn add(self, rhs: &'b $affine) -> $affine {
                self + rhs
            }
        }

        impl AddAssign for $affine {
            fn add_assign(&mut self, rhs: $affine) {
                *self = self.add(rhs);
            }
        }

        impl<'a, 'b> Mul<&'b $field> for &'a $affine {
            type Output = $affine;

            #[inline]
            fn mul(self, scalar: &'b $field) -> $affine {
                let mut res = Self::Output::IDENTITY.to_projective();
                let mut acc = self.clone().to_projective();
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
                $affine::from(res)
            }
        }

        impl MulAssign<$field> for $affine {
            fn mul_assign(&mut self, scalar: $field) {
                *self = *self * scalar;
            }
        }

        impl Neg for $affine {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                    is_infinity: self.is_infinity,
                }
            }
        }

        impl<'a> Neg for &'a $affine {
            type Output = $affine;

            #[inline]
            fn neg(self) -> $affine {
                -self
            }
        }

        impl Sub for $affine {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $affine) -> Self {
                Self::from(self.to_projective() + rhs.neg().to_projective())
            }
        }

        impl<'a, 'b> Sub<&'b $affine> for &'a $affine {
            type Output = $affine;

            #[inline]
            fn sub(self, rhs: &'b $affine) -> $affine {
                self - rhs
            }
        }

        impl SubAssign for $affine {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = self.add(rhs.neg());
            }
        }
    };
}

pub use {affine_ring_operation, projective_ring_operation};
