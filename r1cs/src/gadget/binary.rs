use crate::driver::CircuitDriver;
use crate::matrix::SparseRow;
use crate::wire::Wire;
use crate::R1cs;
use std::ops::{Neg, Sub};

use crate::gadget::field::FieldAssignment;
use zkstd::common::{Add, IntGroup, Nafs, PrimeField};

#[derive(Clone)]
pub struct BinaryAssignment<C: CircuitDriver>(Vec<FieldAssignment<C>>);

impl<C: CircuitDriver> BinaryAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, val: &FieldAssignment<C>) -> Self {
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
                    let bit = FieldAssignment::witness(cs, C::Scalar::from(*w as u64));
                    let res = &acc
                        + &FieldAssignment::mul(
                            cs,
                            &bit,
                            &FieldAssignment::constant(&C::Scalar::pow_of_2(i as u64)),
                        );
                    decomposition.push(bit);
                    res
                },
            );
        FieldAssignment::eq(cs, val, &acc);
        Self(decomposition)
    }

    pub fn get(&self) -> &[FieldAssignment<C>] {
        &self.0
    }
}
