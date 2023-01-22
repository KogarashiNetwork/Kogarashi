mod group;
mod test;

pub use group::*;
pub use test::*;

#[macro_export]
macro_rules! curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $x:ident, $y:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        affine_group_operation!($affine, $projective, $range, $scalar, $x, $y);
        projective_group_operation!($projective, $range, $scalar, $x, $y);
        mixed_curve_operation!($affine, $projective);

        impl ParityCmp for $affine {}
        impl ParityCmp for $projective {}
        impl Basic for $affine {}
        impl Basic for $projective {}

        impl Curve for $affine {
            type Range = $range;

            type Scalar = $scalar;

            const PARAM_A: $range = $a;

            fn is_identity(self) -> bool {
                self.is_infinity
            }

            fn is_on_curve(self) -> bool {
                if self.is_infinity {
                    true
                } else {
                    self.y.square() == self.x.square() * self.x + Self::PARAM_B
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl WeierstrassCurve for $affine {
            const PARAM_B: $range = $b;
        }

        impl Curve for $projective {
            type Range = $range;

            type Scalar = $scalar;

            const PARAM_A: $range = $a;

            fn is_identity(self) -> bool {
                self.z == Self::Range::zero()
            }

            fn is_on_curve(self) -> bool {
                if self.is_identity() {
                    true
                } else {
                    self.y.square() * self.z
                        == self.x.square() * self.x + Self::PARAM_B * self.z.square() * self.z
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl WeierstrassCurve for $projective {
            const PARAM_B: $range = $b;
        }

        impl CurveExtend for $projective {
            type Affine = $affine;

            fn double(self) -> Self {
                double_point(self)
            }

            fn to_affine(self) -> Self::Affine {
                match self.z.invert() {
                    Some(z_inv) => Self::Affine {
                        x: self.x * z_inv,
                        y: self.y * z_inv,
                        is_infinity: false,
                    },
                    None => Self::Affine::ADDITIVE_IDENTITY,
                }
            }
        }

        impl From<$affine> for $projective {
            fn from(a: $affine) -> $projective {
                a.to_projective()
            }
        }

        impl Affine for $affine {}

        impl WeierstrassAffine for $affine {
            type Projective = $projective;

            fn double(self) -> Self::Projective {
                double_point(self.to_projective())
            }

            fn to_projective(self) -> Self::Projective {
                if self.is_identity() {
                    Self::Projective::ADDITIVE_IDENTITY
                } else {
                    Self::Projective {
                        x: self.x,
                        y: self.y,
                        z: Self::Range::one(),
                    }
                }
            }
        }

        impl From<$projective> for $affine {
            fn from(p: $projective) -> $affine {
                p.to_affine()
            }
        }

        impl Projective for $projective {
            fn new(x: Self::Range, y: Self::Range, z: Self::Range) -> Self {
                Self { x, y, z }
            }

            fn get_z(&self) -> Self::Range {
                self.z
            }
        }
    };
}

#[macro_export]
macro_rules! mixed_curve_operation {
    ($affine:ident, $projective:ident) => {
        impl Add<$projective> for $affine {
            type Output = $projective;

            fn add(self, rhs: $projective) -> $projective {
                add_point(self.to_projective(), rhs)
            }
        }

        impl Sub<$projective> for $affine {
            type Output = $projective;

            fn sub(self, rhs: $projective) -> $projective {
                add_point(self.to_projective(), -rhs)
            }
        }

        impl Add<$affine> for $projective {
            type Output = $projective;

            fn add(self, rhs: $affine) -> $projective {
                add_point(self, rhs.to_projective())
            }
        }

        impl Sub<$affine> for $projective {
            type Output = $projective;

            fn sub(self, rhs: $affine) -> $projective {
                add_point(self, -rhs.to_projective())
            }
        }

        impl AddAssign<$affine> for $projective {
            fn add_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, rhs.to_projective())
            }
        }

        impl SubAssign<$affine> for $projective {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, -rhs.to_projective())
            }
        }
    };
}

pub use {curve_operation, mixed_curve_operation};
