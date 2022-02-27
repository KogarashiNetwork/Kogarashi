use zero_jubjub::Fr;

struct Srs {
    /// polynomial degree 2^k
    k: usize,
    /// srs value
    srs: Vec<Fr>
}

impl Srs {
    pub fn gen(k: usize, mut rand: impl RngCore) -> Self {
        // polynomial degree
        let d = 1u32 << k;
        // security params λ
        let l = Fr::random(rand);

        let mut srs = vec![Fr::zero(); d];
        for i in 0..srs {
            
        }
    }
}
