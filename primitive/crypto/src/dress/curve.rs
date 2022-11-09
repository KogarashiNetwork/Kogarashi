mod group;
mod ring;

pub use group::*;
pub use ring::*;

#[macro_export]
macro_rules! curve_operation {
    ($curve:ident, $field:ident, $a:ident, $b:ident, $affine:ident, $projective:ident, $g:ident, $e:ident) => {
        use zero_crypto::arithmetic::coordinate::projective::*;

        curve_built_in!($affine, $projective);

        projective_ring_operation!($projective, $field, $g, $e);

        impl Curve for $curve {
            type ScalarField = $field;

            type Affine = $affine;

            type Projective = $projective;

            const PARAM_A: $field = $a;

            const PARAM_B: $field = $b;
        }

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

            fn is_on_curve(self) -> bool {
                if self.is_infinity {
                    true
                } else {
                    self.y.square() == self.x.square() * self.x + Self::PARAM_B
                }
            }
        }

        impl Projective for $projective {
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
        }
    };
}

pub use curve_operation;
