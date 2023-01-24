mod group;

pub use group::twisted_edwards_affine_group_operation;

#[macro_export]
macro_rules! twisted_edwards_curve_operation {
    ($scalar:ident, $range:ident, $d:ident, $affine:ident, $extend:ident, $x:ident, $y:ident) => {
        use zero_crypto::behave::*;
        use zero_crypto::common::*;

        twisted_edwards_affine_group_operation!($affine, $extend, $range, $scalar, $x, $y);

        impl ParityCmp for $affine {}
        impl Basic for $affine {}

        impl Curve for $affine {
            type Range = $scalar;

            type Scalar = $scalar;

            const PARAM_A: $scalar = $scalar::one();

            fn is_identity(self) -> bool {
                self.x == $scalar::zero() && self.y == $scalar::one()
            }

            fn is_on_curve(self) -> bool {
                unimplemented!()
            }

            fn get_x(&self) -> Self::Range {
                self.x
            }

            fn get_y(&self) -> Self::Range {
                self.y
            }
        }

        impl TwistedEdwardsAffine for $affine {
            type CurveExtend: $extend;

            fn double(self) -> Self::CurveExtend {
                double_point(self.to_extend())
            }

            fn to_extend(self) -> Self::CurveExtend {}
        }

        impl Affine for $affine {}
    };
}

pub use twisted_edwards_curve_operation;
