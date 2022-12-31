#[macro_export]
macro_rules! extension_field_group_operation {
    ($extension_field:ident, $sub_field:ident, $limbs_length:ident) => {
        impl Group for $extension_field {
            type Scalar = $extension_field;

            const GENERATOR: Self = $extension_field::dummy();

            const ADDITIVE_IDENTITY: Self = $extension_field::dummy();

            fn invert(self) -> Option<Self> {
                self.get_invert()
            }

            fn random(mut rand: impl RngCore) -> Self {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = $sub_field::random(&mut rand);
                }
                $extension_field(limbs)
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

            #[inline]
            fn add(self, rhs: &'b $extension_field) -> $extension_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl AddAssign for $extension_field {
            fn add_assign(&mut self, rhs: $extension_field) {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] + rhs.0[i];
                }
                self.0 = limbs
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

        impl<'a> Neg for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn neg(self) -> $extension_field {
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

        impl<'a, 'b> Sub<&'b $extension_field> for &'a $extension_field {
            type Output = $extension_field;

            #[inline]
            fn sub(self, rhs: &'b $extension_field) -> $extension_field {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                $extension_field(limbs)
            }
        }

        impl SubAssign for $extension_field {
            fn sub_assign(&mut self, rhs: $extension_field) {
                let mut limbs: [$sub_field; $limbs_length] = [$sub_field::zero(); $limbs_length];
                for i in 0..$limbs_length {
                    limbs[i] = self.0[i] - rhs.0[i];
                }
                self.0 = limbs
            }
        }
    };
}

pub use extension_field_group_operation;
