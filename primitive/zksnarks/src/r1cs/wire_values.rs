#[cfg(not(feature = "std"))]
use alloc::collections::btree_map::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
use zkstd::common::Field;

use crate::r1cs::wire::Wire;

/// An assignment of wire values, where each value is an element of the field `F`.
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

    pub fn contains(&self, wire: Wire) -> bool {
        self.values.contains_key(&wire)
    }

    pub fn contains_all(&self, wires: &[Wire]) -> bool {
        wires.iter().all(|&wire| self.contains(wire))
    }
}

impl<F: Field> Clone for WireValues<F> {
    fn clone(&self) -> Self {
        WireValues {
            values: self.values.clone(),
        }
    }
}

pub trait Evaluable<F: Field, R> {
    fn evaluate(&self, wire_values: &WireValues<F>) -> R;
}

/// Creates an instance of `WireValues` from the given wires and field element values.
#[macro_export]
macro_rules! values {
    ( $( $wire:expr => $value:expr ),* ) => {
        {
            let mut values = WireValues::new();
            $(
                values.set($wire, $value);
            )*
            values
        }
    }
}
