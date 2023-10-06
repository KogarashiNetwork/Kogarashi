//! This module extends GadgetBuilder with native field arithmetic methods.

use crate::r1cs::expression::Expression;
use crate::r1cs::gadget_builder::GadgetBuilder;
use crate::r1cs::util::concat;
use crate::r1cs::wire_values::WireValues;
use zkstd::common::Field;

#[allow(dead_code)]
impl<F: Field> GadgetBuilder<F> {
    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn product(&mut self, x: &Expression<F>, y: &Expression<F>) -> Expression<F> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product = self.wire();
        let product_exp = Expression::from(product);
        self.assert_product(x, y, &product_exp);

        {
            let x = x.clone();
            let y = y.clone();
            self.generator(
                concat(&[x.dependencies(), y.dependencies()]),
                move |values: &mut WireValues<F>| {
                    let product_value = x.evaluate(values) * y.evaluate(values);
                    values.set(product, product_value);
                },
            );
        }

        product_exp
    }

    /// Returns `1 / x`, assuming `x` is non-zero. If `x` is zero, the gadget will not be
    /// satisfiable.
    pub fn inverse(&mut self, x: &Expression<F>) -> Expression<F> {
        let x_inv = self.wire();
        self.assert_product(x, &Expression::from(x_inv), &Expression::one());

        let x = x.clone();
        self.generator(x.dependencies(), move |values: &mut WireValues<F>| {
            let x_value = x.evaluate(values);
            let inverse_value = x_value.invert().expect("Can't find an inverse element");
            values.set(x_inv, inverse_value);
        });

        x_inv.into()
    }
}
