use crate::circuit::CircuitDriver;
use crate::r1cs::{R1cs, Wire};
use core::marker::PhantomData;

#[derive(Clone)]
pub struct BinaryAssignment<C: CircuitDriver>(Wire, PhantomData<C>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.public_wire();
        cs.x.push(C::Base::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn witness(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.private_wire();
        cs.w.push(C::Base::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn inner(&self) -> &Wire {
        &self.0
    }
}
