#[macro_export]
macro_rules! twisted_edwards_affine_group_operation {
    ($affine:ident, $extend:ident, $range:ident, $scalar:ident, $x:ident, $y:ident) => {
        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Eq for $affine {}

        impl Default for $affine {
            fn default() -> Self {
                Self::ADDITIVE_IDENTITY
            }
        }

        impl $affine {
            pub const ADDITIVE_GENERATOR: Self = Self {
                x: $x,
                y: $y,
                is_infinity: false,
            };

            pub const ADDITIVE_IDENTITY: Self = Self {
                x: $range::zero(),
                y: $range::one(),
            };

            fn zero() -> Self {
                Self::ADDITIVE_IDENTITY
            }

            fn invert(self) -> Option<Self> {
                match self.is_infinity {
                    true => None,
                    false => Some(Self {
                        x: self.x,
                        y: -self.y,
                    }),
                }
            }

            pub fn random(rand: impl RngCore) -> $extend {
                Self::ADDITIVE_GENERATOR * $scalar::random(rand)
            }
        }

        impl Add for $affine {
            type Output = $extend;

            fn add(self, rhs: $affine) -> Self::Output {
                $extend::from(add_point(self.to_projective(), rhs.to_projective()))
            }
        }

        impl Neg for $affine {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: self.y,
                }
            }
        }

        impl Sub for $affine {
            type Output = $extend;

            fn sub(self, rhs: $affine) -> Self::Output {
                $extend::from(add_point(self.to_projective(), rhs.neg().to_projective()))
            }
        }
    };
}

pub use twisted_edwards_affine_group_operation;
