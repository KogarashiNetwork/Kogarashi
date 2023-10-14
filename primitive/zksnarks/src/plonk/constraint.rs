use crate::plonk::wire::PrivateWire;
use zkstd::common::PrimeField;

/// Each gate expression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constraint<Selector: PrimeField> {
    // Selectors
    /// Multiplier
    pub q_m: Selector,
    /// Left wire
    pub q_l: Selector,
    /// Right wire
    pub q_r: Selector,
    /// Output wire
    pub q_o: Selector,
    /// Fourth wire
    pub q_d: Selector,
    /// Constant wire
    pub q_c: Selector,
    /// Arithmetic wire
    pub q_arith: Selector,
    /// Range
    pub q_range: Selector,
    /// Logic
    pub q_logic: Selector,
    /// Fixed base group addition
    pub q_fixed_group_add: Selector,
    /// Variable base group addition
    pub q_variable_group_add: Selector,

    /// Left
    pub w_a: PrivateWire,
    /// Right
    pub w_b: PrivateWire,
    /// Fourth
    pub w_d: PrivateWire,
    /// Output
    pub w_o: PrivateWire,

    /// Public input
    pub public_input: Option<Selector>,
}

impl<F: PrimeField> Default for Constraint<F> {
    fn default() -> Self {
        Self {
            q_m: F::zero(),
            q_l: F::zero(),
            q_r: F::zero(),
            q_o: F::zero(),
            q_d: F::zero(),
            q_c: F::zero(),
            q_arith: F::zero(),
            q_range: F::zero(),
            q_logic: F::zero(),
            q_fixed_group_add: F::zero(),
            q_variable_group_add: F::zero(),
            w_a: PrivateWire::default(),
            w_b: PrivateWire::default(),
            w_d: PrivateWire::default(),
            w_o: PrivateWire::default(),
            public_input: None,
        }
    }
}

#[allow(dead_code)]
impl<F: PrimeField> Constraint<F> {
    fn from_external(mut constraint: Self) -> Self {
        constraint.q_range = F::zero();
        constraint.q_logic = F::zero();
        constraint.q_fixed_group_add = F::zero();
        constraint.q_variable_group_add = F::zero();
        constraint
    }

    pub fn mult(mut self, s: impl Into<F>) -> Self {
        self.q_m = s.into();
        self
    }

    /// Set `s` as the polynomial selector for the left coefficient.
    pub fn left(mut self, s: impl Into<F>) -> Self {
        self.q_l = s.into();
        self
    }

    /// Set `s` as the polynomial selector for the right coefficient.
    pub fn right(mut self, s: impl Into<F>) -> Self {
        self.q_r = s.into();
        self
    }

    /// Set `s` as the polynomial selector for the output coefficient.
    pub fn output(mut self, s: impl Into<F>) -> Self {
        self.q_o = s.into();
        self
    }

    /// Set `s` as the polynomial selector for the fourth (advice) coefficient.
    pub fn fourth(mut self, s: impl Into<F>) -> Self {
        self.q_d = s.into();
        self
    }

    /// Set `s` as the polynomial selector for the constant of the constraint.
    pub fn constant(mut self, s: impl Into<F>) -> Self {
        self.q_c = s.into();
        self
    }

    /// Set `s` as the public input of the constraint evaluation.
    pub fn public(mut self, s: impl Into<F>) -> Self {
        self.public_input = Some(s.into());
        self
    }

    /// Set witness `a` wired to `qM` and `qL`
    pub fn a(mut self, w: PrivateWire) -> Self {
        self.w_a = w;
        self
    }

    /// Set witness `b` wired to `qM` and `qR`
    pub fn b(mut self, w: PrivateWire) -> Self {
        self.w_b = w;
        self
    }

    /// Set witness `o` wired to `qO`
    pub fn o(mut self, w: PrivateWire) -> Self {
        self.w_o = w;
        self
    }

    /// Set witness `d` wired to the fourth/advice `q4` coefficient
    pub fn d(mut self, w: PrivateWire) -> Self {
        self.w_d = w;
        self
    }

    pub fn arithmetic(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_arith = F::one();
        s
    }

    pub fn range(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_range = F::one();
        s
    }

    pub fn logic(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_c = F::one();
        s.q_logic = F::one();
        s
    }

    pub fn logic_xor(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_c = -F::one();
        s.q_logic = -F::one();
        s
    }

    pub fn group_add_curve_scalar(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_fixed_group_add = F::one();
        s
    }

    pub fn group_add_curve_addtion(s: Self) -> Self {
        let mut s = Self::from_external(s);
        s.q_variable_group_add = F::one();
        s
    }
}
