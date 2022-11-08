mod coordinate;

#[macro_export]
macro_rules! curve_operation {
    ($curve:ident, $field:ident, $a:ident, $b:ident) => {
        impl Curve for $curve {
            type ScalarField = $field;

            const PARAM_A = $a;

            const PARAM_B = $b;
        }
    }
}

pub use curve_operation;
