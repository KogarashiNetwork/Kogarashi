use crate::driver::CircuitDriver;
use crate::R1cs;

use crate::gadget::field::FieldAssignment;
use zkstd::common::{IntGroup, PrimeField};

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
                    decomposition.push(bit.clone());
                    let res = &acc
                        + &FieldAssignment::mul(
                            cs,
                            &bit,
                            &FieldAssignment::constant(&C::Scalar::pow_of_2(i as u64)),
                        );
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
        let _bits: BinaryAssignment<GrumpkinDriver> = FieldAssignment::to_bits(&mut cs, &x);

        assert!(cs.is_sat());
    }
}
