use crate::Witness;
use zkstd::common::PrimeField;

pub enum Selector {
    /// Multiplication Selector
    Multiplication = 0x00,
    /// Left Selector
    Left = 0x01,
    /// Right Selector
    Right = 0x02,
    /// Output Selector
    Output = 0x03,
    /// Fourth advice Selector
    Fourth = 0x04,
    /// Constant expression `q_c`
    Constant = 0x05,
    /// Public input `pi`
    PublicInput = 0x06,

    /// Arithmetic Selector
    Arithmetic = 0x07,
    /// Range Selector
    Range = 0x08,
    /// Logic Selector
    Logic = 0x09,
    /// Curve addition with fixed base Selector
    GroupAddFixedBase = 0x0a,
    /// Curve addition with variable base Selector
    GroupAddVariableBase = 0x0b,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wire {
    /// `A` Witness
    A = 0x00,
    /// `B` Witness
    B = 0x01,
    /// `O` Witness
    O = 0x02,
    /// `D` Witness
    D = 0x03,
}

#[derive(Default)]
pub struct Constraint<F: PrimeField> {
    selectors: [F; 13],
    witnesses: [Witness; 4],
    has_public_input: bool,
}

impl<F: PrimeField> Constraint<F> {
    fn enable_selector(&mut self, r: Selector) {
        self.selectors[r as usize] = F::one();
    }

    fn asign_witness(&mut self, wire: Wire, w: Witness) {
        self.witnesses[wire as usize] = w;
    }

    pub fn left(&mut self) {
        self.enable_selector(Selector::Left)
    }

    pub fn right(&mut self) {
        self.enable_selector(Selector::Right)
    }

    pub fn public(&mut self) {
        self.has_public_input = true;

        self.enable_selector(Selector::PublicInput)
    }
}
