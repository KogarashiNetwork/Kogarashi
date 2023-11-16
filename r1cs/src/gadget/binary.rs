use crate::driver::CircuitDriver;
use crate::{R1cs, Wire};
use std::marker::PhantomData;

use crate::gadget::field::FieldAssignment;
use zkstd::common::{IntGroup, PrimeField};

#[derive(Clone)]
pub struct BinaryAssignment<C: CircuitDriver>(Wire, PhantomData<C>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn witness(cs: &mut R1cs<C>, bit: u8) -> Self {
        if bit != 0 && bit != 1 {
            panic!("Bit value should be passed, got {bit}");
        }
        let wire = cs.private_wire();
        cs.w.push(C::Scalar::from(bit as u64));

        Self(wire, PhantomData::default())
    }
    pub fn decomposition(cs: &mut R1cs<C>, val: &FieldAssignment<C>) -> Vec<Self> {
        let mut decomposition = vec![];

        let acc = val
            .inner()
            .evaluate(&cs.x, &cs.w)
            .to_bits()
            .iter()
            .rev()
            .enumerate()
            .fold(
                FieldAssignment::constant(&C::Scalar::zero()),
                |acc, (i, w)| {
                    let bit = BinaryAssignment::witness(cs, *w);
                    decomposition.push(bit.clone());
                    let res = &acc
                        + &FieldAssignment::mul(
                            cs,
                            &FieldAssignment::from(bit),
                            &FieldAssignment::constant(&C::Scalar::pow_of_2(i as u64)),
                        );
                    res
                },
            );
        FieldAssignment::eq(cs, val, &acc);
        decomposition
    }

    pub fn inner(&self) -> &Wire {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GrumpkinDriver;
    use bn_254::Fr;
    use zkstd::common::{Group, OsRng};

    #[test]
    fn binary_assignment_instance() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let input = Fr::random(OsRng);

        let x = FieldAssignment::instance(&mut cs, input);
        let _bits: Vec<BinaryAssignment<GrumpkinDriver>> = FieldAssignment::to_bits(&mut cs, &x);

        assert!(cs.is_sat());
    }
}
