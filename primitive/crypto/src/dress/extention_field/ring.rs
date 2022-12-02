#[macro_export]
macro_rules! extention_field_ring_operation {
    ($extention_field:ident, $sub_field:ident, $limbs_length:ident) => {
        extention_field_group_operation!($extention_field);

        impl Ring for $extention_field {}

        impl Add for $extention_field {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $extention_field) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl<'a, 'b> Add<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn add(self, rhs: &'b $extention_field) -> $extention_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl AddAssign for $extention_field {
            fn add_assign(&mut self, rhs: $extention_field) {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                self.0 = limbs
            }
        }

        impl Neg for $extention_field {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = -self.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl<'a> Neg for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn neg(self) -> $extention_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = -self.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl Sub for $extention_field {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: $extention_field) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl<'a, 'b> Sub<&'b $extention_field> for &'a $extention_field {
            type Output = $extention_field;

            #[inline]
            fn sub(self, rhs: &'b $extention_field) -> $extention_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                $extention_field(limbs)
            }
        }

        impl SubAssign for $extention_field {
            fn sub_assign(&mut self, rhs: $extention_field) {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                self.0 = limbs
            }
        }
    };
}

pub use extention_field_ring_operation;
