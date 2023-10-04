use crate::r1cs::wire::Wire;
use alloc::collections::BTreeMap;
use zkstd::common::Field;

#[derive(Default, Debug)]
pub struct WireValues<F: Field> {
    values: BTreeMap<Wire, F>,
}

impl<F: Field> WireValues<F> {
    pub fn new() -> Self {
        let mut values = BTreeMap::new();
        values.insert(Wire::ONE, F::one());
        WireValues { values }
    }

    pub fn as_map(&self) -> &BTreeMap<Wire, F> {
        &self.values
    }

    pub fn get(&self, wire: Wire) -> &F {
        assert!(self.values.contains_key(&wire), "No value for {}", wire);
        &self.values[&wire]
    }

    pub fn set(&mut self, wire: Wire, value: F) {
        let old_value = self.values.insert(wire, value);
        assert!(old_value.is_none());
    }
}
