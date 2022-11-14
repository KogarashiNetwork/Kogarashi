mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! curve_operation {
    ($field:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $g:ident, $e:ident) => {
        curve_built_in!($affine, $projective);

        projective_ring_operation!($projective, $field, $g, $e);

        impl Affine for $affine {
            type ScalarField = $field;

            type Projective = $projective;

            const PARAM_A: $field = $a;

            const PARAM_B: $field = $b;

            fn to_projective(self) -> Self::Projective {
                Self::Projective {
                    x: self.x,
                    y: self.y,
                    z: Self::ScalarField::one(),
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
            type ScalarField = $field;

            type Affine = $affine;

            const PARAM_A: $field = $a;

            const PARAM_B: $field = $b;

            fn to_affine(self) -> Self::Affine {
                Self::Affine {
                    x: self.x / self.z,
                    y: self.y / self.z,
                    is_infinity: self.z == Self::ScalarField::zero()
                }
            }

            fn is_identity(self) -> bool {
                self.z == Self::ScalarField::zero()
            }

            fn double(self) -> Self {
                let (x,y,z) = double_point((self.x.0, self.y.0, self.z.0), $field::MODULUS.0,
                $field::INV,);
                Self {
                    x: $field(x),
                    y: $field(y),
                    z: $field(z),
                }
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
