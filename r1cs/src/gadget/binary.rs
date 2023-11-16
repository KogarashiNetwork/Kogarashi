use crate::driver::CircuitDriver;
use crate::{R1cs, Wire};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct BinaryAssignment<C: CircuitDriver>(Wire, PhantomData<C>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, bit: u8) -> Self {
        if bit != 0 && bit != 1 {
            panic!("Bit value should be passed, got {bit}");
        }
        let wire = cs.public_wire();
        cs.x.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn witness(cs: &mut R1cs<C>, bit: u8) -> Self {
        if bit != 0 && bit != 1 {
            panic!("Bit value should be passed, got {bit}");
        }
        let wire = cs.private_wire();
        cs.w.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }

    pub fn inner(&self) -> &Wire {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GrumpkinDriver;

    #[test]
    fn binary_assignment_instance() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();

        let _ = BinaryAssignment::instance(&mut cs, 0);
        let _ = BinaryAssignment::instance(&mut cs, 1);
        let _ = BinaryAssignment::witness(&mut cs, 0);
        let _ = BinaryAssignment::witness(&mut cs, 1);

        assert!(cs.is_sat());
    }

    #[test]
    #[should_panic]
    fn binary_assignment_instance_non_bit() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();

        let _ = BinaryAssignment::instance(&mut cs, 2);
    }
}
