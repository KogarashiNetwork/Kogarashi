use merlin::Transcript as Merlin;
use r1cs::CircuitDriver;
use zkstd::common::PrimeField;

pub trait Transcript<C: CircuitDriver> {
    fn absorb(&mut self, label: &'static [u8], value: C::Base);
}

impl<C: CircuitDriver> Transcript<C> for Merlin {
    fn absorb(&mut self, label: &'static [u8], value: C::Base) {
        self.append_message(label, &value.to_raw_bytes())
    }
}
