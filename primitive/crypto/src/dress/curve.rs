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

            const PARAM_A: $range = $a;
            const PARAM_B: $range = $b;

            fn is_identity(self) -> bool {
                self.is_infinity
            }

            // TODO: inefficient
            fn double(self) -> Self {
                Self::from(double_point(self.to_extend()))
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

            fn set_x(&mut self, value: Self::Range) {
                self.x = value;
            }

            fn set_y(&mut self, value: Self::Range) {
                self.y = value;
            }
        }

        impl Curve for $projective {
            type Range = $range;

            const PARAM_A: $range = $a;
            const PARAM_B: $range = $b;

            fn is_identity(self) -> bool {
                self.z == Self::Range::zero()
            }

            fn double(self) -> Self {
                double_point(self)
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

            fn set_x(&mut self, value: Self::Range) {
                self.x = value;
            }

            fn set_y(&mut self, value: Self::Range) {
                self.y = value;
            }
        }

        impl CurveExtend for $projective {
            type Affine = $affine;

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
                a.to_extend()
            }
        }

        impl Affine for $affine {
            type Scalar = $scalar;
            type CurveExtend = $projective;

            fn to_extend(self) -> Self::CurveExtend {
                if self.is_identity() {
                    Self::CurveExtend::ADDITIVE_IDENTITY
                } else {
                    Self::CurveExtend {
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
            fn get_z(&self) -> Self::Range {
                self.z
            }

            fn set_z(&mut self, value: Self::Range) {
                self.z = value;
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
                add_point(self.to_extend(), rhs)
            }
        }

        impl Sub<$projective> for $affine {
            type Output = $projective;

            fn sub(self, rhs: $projective) -> $projective {
                add_point(self.to_extend(), -rhs)
            }
        }

        impl Add<$affine> for $projective {
            type Output = $projective;

            fn add(self, rhs: $affine) -> $projective {
                add_point(self, rhs.to_extend())
            }
        }

        impl Sub<$affine> for $projective {
            type Output = $projective;

            fn sub(self, rhs: $affine) -> $projective {
                add_point(self, -rhs.to_extend())
            }
        }

        impl AddAssign<$affine> for $projective {
            fn add_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, rhs.to_extend())
            }
        }

        impl SubAssign<$affine> for $projective {
            fn sub_assign(&mut self, rhs: $affine) {
                *self = add_point(*self, -rhs.to_extend())
            }
        }
    };
}

pub use {curve_operation, mixed_curve_operation};
