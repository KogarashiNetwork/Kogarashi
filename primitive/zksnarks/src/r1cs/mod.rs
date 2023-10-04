#![allow(dead_code)]
#![allow(unused_variables)]

mod constraint;
mod matrix;
mod wire;
mod wire_values;

use zkstd::common::Field;

struct Expression {}

struct GadgetBuilder {}

impl GadgetBuilder {
    pub fn new() -> Self {
        Self {}
    }

    fn build(&self) -> Gadget {
        Gadget {}
    }
}

struct Gadget {}

impl Gadget {
    fn execute(&self, wire_value: &mut WireValues) -> bool {
        true
    }
}

struct EdwardsExpression {}
impl EdwardsExpression {
    pub fn new(builder: &mut GadgetBuilder, x_exp: Expression, y_exp: Expression) -> Self {
        Self {}
    }
}

struct WireValues {}

impl WireValues {
    pub fn new() -> Self {
        Self {}
    }
}

impl<F: Field> From<F> for Expression {
    fn from(_value: F) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jub_jub::Fr;
    #[test]
    fn point_on_curve() {
        let x = Fr::from_hex("0x187d2619ff114316d237e86684fb6e3c6b15e9b924fa4e322764d3177508297a")
            .unwrap();
        let y = Fr::from_hex("0x6230c613f1b460e026221be21cf4eabd5a8ea552db565cb18d3cabc39761eb9b")
            .unwrap();

        let x_exp = Expression::from(x);
        let y_exp = Expression::from(y);

        let mut builder = GadgetBuilder::new();
        let p = EdwardsExpression::new(&mut builder, x_exp, y_exp);

        let gadget = builder.build();
        assert!(gadget.execute(&mut WireValues::new()));
    }
}
