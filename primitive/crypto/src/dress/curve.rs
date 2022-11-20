mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $g:ident, $e:ident) => {
        curve_built_in!($affine, $projective);

        projective_ring_operation!($projective, $scalar, $g, $e);

        impl Affine for $affine {
            type ScalarField = $scalar;

            type RangeField = $range;

            type Projective = $projective;

            const PARAM_A: $range = $a;

            const PARAM_B: $range = $b;

            fn to_projective(self) -> Self::Projective {
                Self::Projective {
                    x: self.x,
                    y: self.y,
                    z: Self::RangeField::one(),
                }
            }

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
        }

        impl PartialEq for $affine {
            fn eq(&self, other: &Self) -> bool {
                if self.is_identity() || other.is_identity() {
                    self.is_identity() && other.is_identity()
                } else {
                    self.x == other.x && self.y == other.y
                }
            }
        }

        impl Eq for $affine {}

        impl Projective for $projective {
            type ScalarField = $scalar;

            type RangeField = $range;

            type Affine = $affine;

            const PARAM_A: $range = $a;

            const PARAM_B: $range = $b;

            fn to_affine(self) -> Self::Affine {
                let inv_z = self.z.invert().unwrap();
                Self::Affine {
                    x: self.x * inv_z,
                    y: self.y * inv_z,
                    is_infinity: self.z == Self::RangeField::zero()
                }
            }

            fn is_identity(self) -> bool {
                self.z == Self::RangeField::zero()
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

            fn get_x(&self) -> Self::RangeField {
                self.x
            }

            fn get_y(&self) -> Self::RangeField {
                self.y
            }

            fn get_z(&self) -> Self::RangeField {
                self.z
            }

            fn set_x(&mut self, value: Self::RangeField) {
                self.x = value;
            }

            fn set_y(&mut self, value: Self::RangeField) {
               self.y = value;
            }

            fn set_z(&mut self, value: Self::RangeField) {
                self.z = value;
            }
        }
    };
}

#[macro_export]
macro_rules! curve_built_in {
    ($affine:ident, $projective:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        impl ParityCmp for $affine {}

        impl ParityCmp for $projective {}

        impl Basic for $affine {}

        impl Basic for $projective {}

        impl Default for $affine {
            fn default() -> Self {
                unimplemented!()
            }
        }

        impl Default for $projective {
            fn default() -> Self {
                Self::GENERATOR
            }
        }

        impl Display for $affine {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "x: 0x")?;
                for i in self.x.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
        impl Display for $projective {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "x: 0x")?;
                for i in self.x.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                write!(f, " z: 0x")?;
                for i in self.z.0.iter().rev() {
                    write!(f, "{:016x}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use {curve_built_in, curve_operation};
