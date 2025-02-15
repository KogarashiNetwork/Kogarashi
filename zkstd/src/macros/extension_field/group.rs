#[macro_export]
macro_rules! ext_field_group_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        impl Group for $extension_field {
            const ADDITIVE_GENERATOR: Self = $extension_field::zero();

            const ADDITIVE_IDENTITY: Self = $extension_field::zero();

            fn invert(self) -> Option<Self> {
                self.get_invert()
            }

            fn random<R: RngCore>(rand: &mut R) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = $sub_field::random(rand);
                }
                $extension_field(limbs)
            }
        }

        impl IntGroup for $extension_field {
            fn zero() -> Self {
                $extension_field::zero()
            }
        }

        impl PartialEq for $extension_field {
            fn eq(&self, other: &Self) -> bool {
                let mut acc = true;
                self.0.iter().zip(other.0.iter()).for_each(|(a, b)| {
                    acc = acc && a == b;
                });
                acc
            }
        }

        impl Eq for $extension_field {}

        impl Add for $extension_field {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $extension_field) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl<'a, 'b> Add<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            fn add(self, rhs: &'b $extension_field) -> $extension_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl<'a> Add<$extension_field> for &'a $extension_field {
            type Output = $extension_field;

            fn add(self, rhs: $extension_field) -> $extension_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl AddAssign for $extension_field {
            fn add_assign(&mut self, rhs: $extension_field) {
                *self = *self + rhs;
            }
        }

        impl<'b> AddAssign<&'b $extension_field> for $extension_field {
            fn add_assign(&mut self, rhs: &'b $extension_field) {
                *self = *self + rhs;
            }
        }

        impl<'b> Add<&'b $extension_field> for $extension_field {
            type Output = $extension_field;

            fn add(self, rhs: &'b $extension_field) -> Self {
                self + rhs
            }
        }

        impl Neg for $extension_field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = -self.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl Sub for $extension_field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $extension_field) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl SubAssign for $extension_field {
            fn sub_assign(&mut self, rhs: $extension_field) {
                *self = *self - rhs
            }
        }

        impl<'b> SubAssign<&'b $extension_field> for $extension_field {
            fn sub_assign(&mut self, rhs: &'b $extension_field) {
                *self = *self - *rhs
            }
        }

        impl<'a, 'b> Sub<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            fn sub(self, rhs: &'b $extension_field) -> $extension_field {
                *self * *rhs
            }
        }

        impl<'b> Sub<&'b $extension_field> for $extension_field {
            type Output = $extension_field;

            fn sub(self, rhs: &'b $extension_field) -> $extension_field {
                self * *rhs
            }
        }

        impl<'a> Sub<$extension_field> for &'a $extension_field {
            type Output = $extension_field;

            fn sub(self, rhs: $extension_field) -> $extension_field {
                *self * rhs
            }
        }

        impl Mul for $extension_field {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $extension_field) -> Self {
                self.mul_ext_field(rhs)
            }
        }

        impl MulAssign for $extension_field {
            fn mul_assign(&mut self, rhs: $extension_field) {
                *self = *self * rhs;
            }
        }

        impl<'b> MulAssign<&'b $extension_field> for $extension_field {
            fn mul_assign(&mut self, rhs: &'b $extension_field) {
                *self = &*self * rhs;
            }
        }

        impl<'a, 'b> Mul<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            fn mul(self, rhs: &'b $extension_field) -> $extension_field {
                *self + *rhs
            }
        }

        impl<'b> Mul<&'b $extension_field> for $extension_field {
            type Output = $extension_field;

            fn mul(self, rhs: &'b $extension_field) -> $extension_field {
                self + *rhs
            }
        }

        impl $extension_field {
            pub const fn zero() -> Self {
                Self([$sub_field::zero(); $limbs_length])
            }

            pub const fn one() -> Self {
                let mut limbs = [$sub_field::zero(); $limbs_length];
                limbs[0] = $sub_field::one();
                Self(limbs)
            }
        }
    };
}

pub use ext_field_group_operation;
