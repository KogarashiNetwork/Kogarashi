use generic_array::typenum::U24;
use neptune::{
    circuit2::Elt,
    poseidon::PoseidonConstants,
    sponge::{
        api::{IOPattern, SpongeAPI, SpongeOp},
        circuit::SpongeCircuit,
        vanilla::{Mode::Simplex, Sponge, SpongeTrait},
    },
    Strength,
};
use r1cs::{CircuitDriver, R1cs};
use std::marker::PhantomData;
use zkstd::common::{Deserialize, PrimeField, Serialize};

pub(crate) const NUM_CHALLENGE_BITS: usize = 128;

/// All Poseidon Constants that are used in Nova
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseidonConstantsCircuit<Scalar: PrimeField + ff::PrimeField>(
    PoseidonConstants<Scalar, U24>,
);

impl<Scalar: PrimeField + ff::PrimeField> Default for PoseidonConstantsCircuit<Scalar> {
    /// Generate Poseidon constants
    fn default() -> Self {
        Self(Sponge::<Scalar, U24>::api_constants(Strength::Standard))
    }
}

/// A Poseidon-based RO to use outside circuits
#[derive(Serialize, Deserialize)]
pub struct PoseidonRO<Base, Scalar>
where
    Base: PrimeField + ff::PrimeField,
    Scalar: PrimeField + ff::PrimeField,
{
    // Internal State
    state: Vec<Base>,
    constants: PoseidonConstantsCircuit<Base>,
    num_absorbs: usize,
    squeezed: bool,
    _p: PhantomData<Scalar>,
}

impl<Base, Scalar> PoseidonRO<Base, Scalar>
where
    Base: PrimeField + Serialize + for<'de> Deserialize<'de> + ff::PrimeField + ff::PrimeFieldBits,
    Scalar: PrimeField + ff::PrimeField,
{
    fn new(constants: PoseidonConstantsCircuit<Base>, num_absorbs: usize) -> Self {
        Self {
            state: Vec::new(),
            constants,
            num_absorbs,
            squeezed: false,
            _p: PhantomData,
        }
    }

    /// Absorb a new number into the state of the oracle
    fn absorb(&mut self, e: Base) {
        assert!(!self.squeezed, "Cannot absorb after squeezing");
        self.state.push(e);
    }

    /// Compute a challenge by hashing the current state
    fn squeeze(&mut self, num_bits: usize) -> Scalar {
        // check if we have squeezed already
        assert!(!self.squeezed, "Cannot squeeze again after squeezing");
        self.squeezed = true;

        let mut sponge = Sponge::new_with_constants(&self.constants.0, Simplex);
        let acc = &mut ();
        let parameter = IOPattern(vec![
            SpongeOp::Absorb(self.num_absorbs as u32),
            SpongeOp::Squeeze(1u32),
        ]);

        sponge.start(parameter, None, acc);
        assert_eq!(self.num_absorbs, self.state.len());
        SpongeAPI::absorb(&mut sponge, self.num_absorbs as u32, &self.state, acc);
        let hash = SpongeAPI::squeeze(&mut sponge, 1, acc);
        sponge.finish(acc).unwrap();

        // Only return `num_bits`
        let bits = hash[0].to_le_bits();
        let mut res = Scalar::zero();
        let mut coeff = Scalar::one();
        for bit in bits[0..num_bits].into_iter() {
            if *bit {
                res += coeff;
            }
            coeff += coeff;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use r1cs::GrumpkinDriver;
    use rand_core::OsRng;
    use zkstd::common::Group;

    fn test_poseidon_ro_with<C: CircuitDriver>() {
        // Check that the number computed inside the circuit is equal to the number computed outside the circuit
        let mut csprng: OsRng = OsRng;
        let constants = PoseidonConstantsCircuit::<C::Scalar>::default();
        let num_absorbs = 32;
        let mut ro: PoseidonRO<C::Scalar, C::Base> =
            PoseidonRO::new(constants.clone(), num_absorbs);

        for i in 0..num_absorbs {
            let num = C::Scalar::random(&mut csprng);
            ro.absorb(num);
        }
        let num = ro.squeeze(NUM_CHALLENGE_BITS);
    }

    #[test]
    fn test_poseidon_ro() {
        test_poseidon_ro_with::<GrumpkinDriver>();
    }
}
