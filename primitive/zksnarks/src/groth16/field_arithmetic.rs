//! This module extends GadgetBuilder with native field arithmetic methods.

use super::constraint_system::ConstraintSystem;
use super::expression::Expression;
use zkstd::common::{Group, TwistedEdwardsAffine};

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> {
    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn product(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
    ) -> Expression<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value =
            x.evaluate(&self.instance, &self.witness) * y.evaluate(&self.instance, &self.witness);
        let product = self.alloc_public(product_value);
        let product_exp = Expression::from(product);
        self.assert_product(x, y, &product_exp);

        product_exp
    }

    /// Returns `1 / x`, assuming `x` is non-zero. If `x` is zero, the gadget will not be
    /// satisfiable.
    pub fn inverse(&mut self, x: &Expression<C::Range>) -> Expression<C::Range> {
        let x_value = x.evaluate(&self.instance, &self.witness);
        let inverse_value = x_value.invert().expect("Can't find an inverse element");
        let x_inv = self.alloc_public(inverse_value);

        let x_inv_expression = Expression::from(x_inv);
        self.assert_product(x, &x_inv_expression, &Expression::one());

        x_inv_expression
    }
}
