mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! curve_operation {
    ($scalar:ident, $range:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $g:ident, $e:ident) => {
        curve_built_in!($affine, $projective);
        affine_group_operation!($affine, $scalar,$g, $e);
        projective_group_operation!($projective, $scalar, $g, $e);

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

        impl Affine for $affine {
            type Projective = $projective;

            fn to_projective(self) -> Self::Projective {
                Self::Projective {
                    x: self.x,
                    y: self.y,
                    z: Self::Range::one(),
                }
            }
        }

        impl Mul<$scalar> for $affine {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $scalar) -> Self {
                self * scalar
            }
        }

        impl Projective for $projective {
            type Affine = $affine;

            fn to_affine(self) -> Self::Affine {
                let inv_z = self.z.invert().unwrap();
                Self::Affine {
                    x: self.x * inv_z,
                    y: self.y * inv_z,
                    is_infinity: self.z == Self::Range::zero()
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

        impl Mul<$scalar> for $projective {
            type Output = Self;

            #[inline]
            fn mul(self, scalar: $scalar) -> Self {
                let mut res = Self::Output::ADDITIVE_IDENTITY;
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
                    acc = acc.double();
                }
                res
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
                $projective::GENERATOR.to_affine()
            }
        }

        impl Default for $projective {
            fn default() -> Self {
                Self::GENERATOR
            }
        }

        impl From<$affine> for $projective {
            fn from(a: $affine) -> $projective {
                a.to_projective()
            }
        }

        impl From<$projective> for $affine {
            fn from(p: $projective) -> $affine {
                p.to_affine()
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
                    write!(f, "{:?}", *i)?;
                }
                write!(f, " y: 0x")?;
                for i in self.y.0.iter().rev() {
                    write!(f, "{:?}", *i)?;
                }
                write!(f, " z: 0x")?;
                for i in self.z.0.iter().rev() {
                    write!(f, "{:?}", *i)?;
                }
                Ok(())
            }
        }
    };
}

pub use {curve_built_in, curve_operation};
