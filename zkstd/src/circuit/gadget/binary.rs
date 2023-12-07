use crate::circuit::prelude::FieldAssignment;
use crate::circuit::CircuitDriver;
use crate::common::{IntGroup, Ring};
use crate::r1cs::{R1cs, Wire};
use core::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct BinaryAssignment<C: CircuitDriver>(Wire, PhantomData<C>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.public_wire();
        cs.x.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn conditional_enforce_equal(cs: &mut R1cs<C>, x: &Self, y: &Self, should_enforce: &Self) {
        FieldAssignment::conditional_enforce_equal(
            cs,
            &FieldAssignment::from(x),
            &FieldAssignment::from(y),
            should_enforce,
        );
    }

    pub fn witness(cs: &mut R1cs<C>, bit: u8) -> Self {
        let wire = cs.private_wire();
        cs.w.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    // TODO: Think about the way to do it without new allocation
    pub fn not(cs: &mut R1cs<C>, b: &Self) -> Self {
        let wire = cs.private_wire();
        let new_val = if cs[b.0] == C::Scalar::one() {
            C::Scalar::zero()
        } else {
            C::Scalar::one()
        };

        cs.w.push(new_val);

        Self(wire, PhantomData::default())
    }

    // TODO: Do without allocations
    pub fn and(cs: &mut R1cs<C>, a: &Self, b: &Self) -> Self {
        let wire = cs.private_wire();

        let a_and_b = if cs[a.0] == C::Scalar::one() && cs[b.0] == C::Scalar::one() {
            C::Scalar::one()
        } else {
            C::Scalar::zero()
        };

        cs.w.push(a_and_b);

        Self(wire, PhantomData::default())
    }

    pub fn inner(&self) -> &Wire {
        &self.0
    }
}
