mod helper;

use helper::BlakeHelper;
use zkstd::circuit::CircuitDriver;
use zkstd::common::{CurveGroup, IntGroup, PrimeField, Ring};

/// Amount of rounds calculated for the 254 bit field.
/// Doubled due to the usage of Feistel mode with zero key.
pub(crate) const MIMC_ROUNDS: usize = 322;

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
            let mut ccxl = cxl.square();
            ccxl *= cxl;
            ccxl += xr;
            xr = xl;
            xl = ccxl;
        }
        xl
    }
}

pub(crate) struct MimcRO<const ROUND: usize, C: CircuitDriver> {
    hasher: Mimc<ROUND, C::Scalar>,
    state: Vec<C::Scalar>,
    key: C::Scalar,
}

impl<const ROUND: usize, C: CircuitDriver> Default for MimcRO<ROUND, C> {
    fn default() -> Self {
        Self {
            hasher: Mimc::default(),
            state: Vec::default(),
            key: C::Scalar::zero(),
        }
    }
}

impl<const ROUND: usize, C: CircuitDriver> MimcRO<ROUND, C> {
    pub(crate) fn append(&mut self, absorb: C::Scalar) {
        self.state.push(absorb)
    }

    pub(crate) fn append_point(&mut self, point: C::Affine) {
        self.append(point.get_x().into());
        self.append(point.get_y().into());
        self.append(if point.is_identity() {
            C::Scalar::zero()
        } else {
            C::Scalar::one()
        });
    }

    pub(crate) fn hash_vec(&mut self, values: Vec<C::Scalar>) -> C::Scalar {
        for x in values {
            self.state.push(x);
        }
        self.squeeze()
    }

    pub(crate) fn squeeze(&self) -> C::Scalar {
        self.state.iter().fold(self.key, |acc, scalar| {
            let h = self.hasher.hash(*scalar, acc);
            acc + scalar + h
        })
    }
}
