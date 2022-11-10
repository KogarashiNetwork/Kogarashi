mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! curve_operation {
    ($curve:ident, $field:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $g:ident, $e:ident, $mont:ident, $bits:ident) => {
        curve_built_in!($affine, $projective);

        projective_ring_operation!($projective, $field, $g, $e);

        impl Affine<$mont, $bits> for $affine {
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

            fn is_on_curve(self) -> bool {
                if self.is_infinity {
                    true
                } else {
                    self.y.square() == self.x.square() * self.x + Self::PARAM_B
                }
            }
        }

        impl Projective<$mont, $bits> for $projective {
            type ScalarField = $field;

            type Affine = $affine;

            const PARAM_A: $field = $a;

            const PARAM_B: $field = $b;

            fn to_affine(self) -> Self::Affine {
                let inv_z = self.z.invert().unwrap();
                Self::Affine {
                    x: self.x * inv_z,
                    y: self.y * inv_z,
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

pub use curve_operation;
