mod helper;

use helper::BlakeHelper;
use zkstd::circuit::CircuitDriver;
use zkstd::common::{BNAffine, IntGroup, PrimeField, Ring};

/// Amount of rounds calculated for the 254 bit field.
/// Doubled due to the usage of Feistel mode with zero key.
pub(crate) const MIMC_ROUNDS: usize = 46;
/// Because we start with u equals 0 or 1, we have (1 << 125) steps.
/// Until the value of u will reach the MODULUS of the field.
pub(crate) const CHALLENGE_BITS: usize = 128;
pub(crate) const HASH_BITS: usize = 252;

pub(crate) struct Mimc<const ROUND: usize, F: PrimeField> {
    pub(crate) constants: [F; ROUND],
}

impl<const ROUND: usize, F: PrimeField> Default for Mimc<ROUND, F> {
    fn default() -> Self {
        let mut constants = [F::zero(); ROUND];
        let mut helper = BlakeHelper::default();
        for constant in constants.iter_mut() {
            let bytes = helper.get();
            helper.update(&bytes);
            *constant = helper.finalize()
        }

        Self { constants }
    }
}

impl<const ROUND: usize, F: PrimeField> Mimc<ROUND, F> {
    pub(crate) fn hash(&self, mut xl: F, mut xr: F) -> F {
        for c in self.constants {
            let mut cxl = xl;
            cxl += c;
            let ccxl = cxl.square();
            let cccxl = ccxl.square();
            let mut ccccxl = ccxl * cccxl;
            ccccxl *= cxl;
            ccccxl += xr;
            xr = xl;
            xl = ccccxl;
        }
        xl
    }
}

pub(crate) struct MimcRO<const ROUND: usize, C: CircuitDriver> {
    hasher: Mimc<ROUND, C::Base>,
    state: Vec<C::Base>,
    key: C::Base,
}

impl<const ROUND: usize, C: CircuitDriver> Default for MimcRO<ROUND, C> {
    fn default() -> Self {
        Self {
            hasher: Mimc::default(),
            state: Vec::default(),
            key: C::Base::zero(),
        }
    }
}

impl<const ROUND: usize, C: CircuitDriver> MimcRO<ROUND, C> {
    pub(crate) fn append(&mut self, absorb: C::Base) {
        self.state.push(absorb)
    }

    pub(crate) fn append_point(&mut self, point: impl BNAffine<Base = C::Base>) {
        self.append(point.get_x());
        self.append(point.get_y());
        self.append(if point.is_identity() {
            C::Base::zero()
        } else {
            C::Base::one()
        });
    }

    pub(crate) fn append_vec(&mut self, values: Vec<C::Base>) {
        for x in values {
            self.append(x);
        }
    }

    pub(crate) fn squeeze(&self, num_bits: usize) -> C::Scalar {
        let hash = self.state.iter().fold(self.key, |acc, scalar| {
            let h = self.hasher.hash(*scalar, acc);
            acc + scalar + h
        });
        let input_bits = hash.to_bits();
        let mut mult = C::Scalar::one();
        let mut val = C::Scalar::zero();
        for bit in input_bits.iter().rev().take(num_bits) {
            if *bit == 1 {
                val += mult;
            }
            mult = mult + mult;
        }
        val
    }
}
