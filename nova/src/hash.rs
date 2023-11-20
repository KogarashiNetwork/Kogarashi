mod helper;

use helper::BlakeHelper;
use zkstd::common::PrimeField;

pub(crate) struct Mimc<const ROUND: usize, F: PrimeField> {
    constants: [F; ROUND],
}

impl<const ROUND: usize, F: PrimeField> Mimc<ROUND, F> {
    pub(crate) fn new() -> Self {
        let mut constants = [F::zero(); ROUND];
        let mut helper = BlakeHelper::default();
        for constant in constants.iter_mut() {
            let bytes = helper.get();
            helper.update(&bytes);
            *constant = helper.finalize()
        }

        Self { constants }
    }

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
