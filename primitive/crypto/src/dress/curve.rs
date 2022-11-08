mod group;
mod ring;

#[macro_export]
macro_rules! curve_operation {
    ($curve:ident, $field:ident, $a:ident, $b:ident, $projective:ident, $affine:ident) => {
        use zero_crypto::arithmetic::coordinate::projective::*;

        impl Curve for $curve {
            type ScalarField = $field;

            const PARAM_A = $a;

            const PARAM_B = $b;
        }

        impl Projective for $projective {

        }
    }
}

pub use curve_operation;
