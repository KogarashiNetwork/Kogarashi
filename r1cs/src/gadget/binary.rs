use crate::driver::CircuitDriver;
use crate::{R1cs, Wire};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct BinaryAssignment<C: CircuitDriver>(Wire, PhantomData<C>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.public_wire();
        cs.x.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn witness(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.private_wire();
        cs.w.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn inner(&self) -> &Wire {
        &self.0
    }
}
