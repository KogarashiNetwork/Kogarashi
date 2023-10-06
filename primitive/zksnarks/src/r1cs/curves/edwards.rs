use zkstd::common::{Field, TwistedEdwardsAffine};

use crate::r1cs::expression::Expression;
use crate::r1cs::gadget_builder::GadgetBuilder;
use crate::r1cs::wire_values::{Evaluable, WireValues};

impl<C: TwistedEdwardsAffine> Clone for EdwardsExpression<C> {
    fn clone(&self) -> Self {
        EdwardsExpression {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

pub struct EdwardsExpression<C: TwistedEdwardsAffine> {
    pub x: Expression<C::Range>,
    pub y: Expression<C::Range>,
}

#[allow(dead_code)]
impl<C: TwistedEdwardsAffine> EdwardsExpression<C> {
    /// Safely creates an `EdwardsExpression` from two coordinates of type `EdwardsExpression`.
    /// Automatically generates constraints that assert that the resulting curve point
    /// is contained in the EdwardsCurve.
    pub fn new(
        builder: &mut GadgetBuilder<C::Range>,
        x: Expression<C::Range>,
        y: Expression<C::Range>,
    ) -> EdwardsExpression<C> {
        let x_squared = builder.product(&x, &x);
        let y_squared = builder.product(&y, &y);
        let x_squared_y_squared = builder.product(&x_squared, &y_squared);

        builder.assert_equal(
            &y_squared,
            &(Expression::one() + x_squared_y_squared * C::PARAM_D + &x_squared),
        );

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Creates an `EdwardsExpression` from two arbitrary coordinates of type `Expression`.
    /// This method is unsafe and should only be used when the coordinates are proven
    /// to exist on the curve.
    pub fn new_unsafe(x: Expression<C::Range>, y: Expression<C::Range>) -> EdwardsExpression<C> {
        EdwardsExpression { x, y }
    }
}

impl<F: Field, C: TwistedEdwardsAffine<Range = F>> Evaluable<F, C> for EdwardsExpression<C> {
    fn evaluate(&self, wire_values: &WireValues<F>) -> C {
        C::from_raw_unchecked(self.x.evaluate(wire_values), self.y.evaluate(wire_values))
    }
}

#[cfg(test)]
mod tests {
    use crate::r1cs::curves::edwards::EdwardsExpression;
    use crate::r1cs::expression::Expression;
    use crate::r1cs::gadget_builder::GadgetBuilder;
    use crate::r1cs::wire_values::WireValues;
    use jub_jub::{Fr, JubjubAffine};

    #[test]
    fn point_on_curve() {
        let x = Fr::from_hex("0x187d2619ff114316d237e86684fb6e3c6b15e9b924fa4e322764d3177508297a")
            .unwrap();
        let y = Fr::from_hex("0x6230c613f1b460e026221be21cf4eabd5a8ea552db565cb18d3cabc39761eb9b")
            .unwrap();

        let x_exp = Expression::from(x);
        let y_exp = Expression::from(y);

        let mut builder = GadgetBuilder::<Fr>::new();
        let _ = EdwardsExpression::<JubjubAffine>::new(&mut builder, x_exp, y_exp);

        let gadget = builder.build();
        assert!(gadget.execute(&mut WireValues::new()));
    }
}
