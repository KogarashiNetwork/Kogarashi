mod group;
mod test;

pub use group::*;
pub use test::*;

#[macro_export]
macro_rules! curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $x:ident, $y:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        affine_group_operation!($affine, $range, $scalar, $x, $y);
        projective_group_operation!($projective, $range, $scalar, $x, $y);

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

            fn double(self) -> Self {
                Self::from(double_point(self.to_projective()))
            }

            fn is_on_curve(self) -> bool {
                if self.is_infinity {
                    true
                } else {
                    self.y.square() == self.x.square() * self.x + Self::PARAM_B
                }
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
        }

        impl From<$affine> for $projective {
            fn from(a: $affine) -> $projective {
                a.to_projective()
            }
        }

        impl Affine for $affine {
            type Projective = $projective;

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
            type Affine = $affine;

            fn to_affine(self) -> Self::Affine {
                match self.z.invert() {
                    Some(z_inv) => Self::Affine {
                        x: self.x * z_inv,
                        y: self.y * z_inv,
                        is_infinity: self.z == Self::Range::zero(),
                    },
                    None => Self::Affine::ADDITIVE_IDENTITY,
                }
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }

            fn get_z(&self) -> Self::Range {
                self.z
            }

            fn set_x(&mut self, value: Self::Range) {
                self.x = value;
            }

            fn set_y(&mut self, value: Self::Range) {
                self.y = value;
            }

            fn set_z(&mut self, value: Self::Range) {
                self.z = value;
            }
        }
    };
}

pub use curve_operation;
