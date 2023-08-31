use zkstd::behave::PrimeField;

/// Each gate expression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gate<Selector: PrimeField> {
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
    pub w_a: Witness,
    /// Right
    pub w_b: Witness,
    /// Fourth
    pub w_d: Witness,
    /// Output
    pub w_o: Witness,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Witness(usize);

impl Witness {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}
