use rand_core::RngCore;
use sp_std::vec;
use sp_std::vec::Vec;
use zero_jubjub::Fr;

struct Srs {
    /// polynomial degree 2^k
    k: usize,
    /// srs value
    srs: Vec<Fr>,
}

impl Srs {
    pub fn gen(k: usize, mut rand: impl RngCore) -> Self {
        // polynomial degree
        let d = 1usize << k;
        // security params λ
        let l = Fr::random(rand);

        let mut srs = vec![Fr::zero(); d];
        for i in srs.iter() {}
        Self {
            k: 0,
            srs: vec![Fr::zero(); d],
        }
    }
}
