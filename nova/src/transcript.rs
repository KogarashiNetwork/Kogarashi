use generic_array::typenum::U24;
use merlin::Transcript as Merlin;
use neptune::{
    poseidon::PoseidonConstants,
    sponge::{
        api::{IOPattern, SpongeAPI, SpongeOp},
        vanilla::{Mode::Simplex, Sponge, SpongeTrait},
    },
    Strength,
};
use r1cs::CircuitDriver;
use std::marker::PhantomData;
use zkstd::common::{CurveGroup, Deserialize, IntGroup, PrimeField, Ring, Serialize};

pub trait Transcript<C: CircuitDriver> {
    fn absorb(&mut self, label: &'static [u8], value: C::Base);

    fn absorb_point(&mut self, label: &'static [u8], point: C::Affine);

    fn challenge_scalar(&mut self, label: &'static [u8]) -> C::Scalar;
}

impl<C: CircuitDriver> Transcript<C> for Merlin {
    fn absorb(&mut self, label: &'static [u8], value: C::Base) {
        self.append_message(label, &value.to_raw_bytes())
    }

    fn absorb_point(&mut self, label: &'static [u8], point: C::Affine) {
        <Self as Transcript<C>>::absorb(self, label, point.get_x());
        <Self as Transcript<C>>::absorb(self, label, point.get_y());
        <Self as Transcript<C>>::absorb(
            self,
            label,
            if point.is_identity() {
                C::Base::one()
            } else {
                C::Base::zero()
            },
        );
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> C::Scalar {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        C::Scalar::from_bytes_wide(&buf)
    }
}

pub(crate) const NUM_CHALLENGE_BITS: usize = 128;
pub(crate) const NUM_FE_FOR_RO: usize = 24;

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
    pub fn new(constants: PoseidonConstantsCircuit<Base>, num_absorbs: usize) -> Self {
        Self {
            state: Vec::new(),
            constants,
            num_absorbs,
            squeezed: false,
            _p: PhantomData,
        }
    }

    /// Absorb a new number into the state of the oracle
    pub fn absorb(&mut self, e: Base) {
        assert!(!self.squeezed, "Cannot absorb after squeezing");
        self.state.push(e);
    }

    /// Compute a challenge by hashing the current state
    pub fn squeeze(&mut self, num_bits: usize) -> Scalar {
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

pub fn absorb_commitment_in_ro<C: CircuitDriver>(
    comm: C::Affine,
    ro: &mut PoseidonRO<C::Base, C::Scalar>,
) {
    let (x, y, is_infinity) = (comm.get_x(), comm.get_y(), comm.is_identity());
    ro.absorb(x);
    ro.absorb(y);
    ro.absorb(if is_infinity {
        C::Base::one()
    } else {
        C::Base::zero()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use r1cs::{CircuitDriver, GrumpkinDriver};
    use rand_core::OsRng;
    use zkstd::common::Group;

    fn test_poseidon_ro_with<C: CircuitDriver>() {
        let constants = PoseidonConstantsCircuit::<C::Scalar>::default();
        let num_absorbs = 32;
        let mut ro: PoseidonRO<C::Scalar, C::Base> = PoseidonRO::new(constants, num_absorbs);

        for _ in 0..num_absorbs {
            let num = C::Scalar::random(OsRng);
            ro.absorb(num);
        }
        let _ = ro.squeeze(NUM_CHALLENGE_BITS);
    }

    #[test]
    fn test_poseidon_ro() {
        test_poseidon_ro_with::<GrumpkinDriver>();
    }
}
