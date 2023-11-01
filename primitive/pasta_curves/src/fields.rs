mod fp;
mod fq;

pub use fp::Fp;
pub use fq::Fq;
use zkstd::common::SigUtils;

impl From<Fq> for Fp {
    fn from(el: Fq) -> Fp {
        let val = Fp::from_bytes(el.to_bytes());

        assert!(
            val.is_some(),
            "Failed to convert a Scalar from Bls to Jubjub"
        );

        val.unwrap()
    }
}

impl From<Fp> for Fq {
    fn from(el: Fp) -> Fq {
        let val = Fq::from_bytes(el.to_bytes());

        assert!(
            val.is_some(),
            "Failed to convert a Scalar from Bls to Jubjub"
        );

        val.unwrap()
    }
}
