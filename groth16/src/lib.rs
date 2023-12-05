#![doc = include_str!("../README.md")]

mod circuit;
mod error;
mod fft;
mod msm;
mod params;
mod poly;
mod proof;
mod prover;
mod verifier;
mod zksnark;

pub use proof::Proof;
pub use prover::Prover;
pub use verifier::Verifier;
pub use zksnark::ZkSnark;

#[cfg(test)]
mod tests {
    use crate::circuit::Circuit;
    use crate::error::Error;
    use crate::zksnark::ZkSnark;

    use bn_254::Fr as GrumpkinBase;
    use grumpkin::driver::GrumpkinDriver;
    use zkstd::circuit::prelude::{FieldAssignment, R1cs};
    use zkstd::common::OsRng;

    #[test]
    fn arithmetic_test() {
        #[derive(Debug)]
        pub struct DummyCircuit {
            x: GrumpkinBase,
            o: GrumpkinBase,
        }

        impl DummyCircuit {
            pub fn new(x: GrumpkinBase, o: GrumpkinBase) -> Self {
                Self { x, o }
            }
        }

        impl Default for DummyCircuit {
            fn default() -> Self {
                Self::new(0.into(), 0.into())
            }
        }

        impl Circuit for DummyCircuit {
            fn synthesize(&self, composer: &mut R1cs<GrumpkinDriver>) -> Result<(), Error> {
                let x = FieldAssignment::instance(composer, self.x);
                let o = FieldAssignment::instance(composer, self.o);
                let c = FieldAssignment::constant(&GrumpkinBase::from(5));

                let sym1 = FieldAssignment::mul(composer, &x, &x);
                let y = FieldAssignment::mul(composer, &sym1, &x);
                // TODO: check why using the `Add` trait crashes this test
                let sym2 = FieldAssignment::add(composer, &y, &x);

                FieldAssignment::eq(composer, &(&sym2 + &c), &o);

                Ok(())
            }
        }

        let x = GrumpkinBase::from(3);
        let o = GrumpkinBase::from(35);
        let circuit = DummyCircuit::new(x, o);

        let (mut prover, verifier) =
            ZkSnark::setup::<DummyCircuit>(OsRng).expect("Failed to compile circuit");
        let proof = prover
            .create_proof(&mut OsRng, circuit)
            .expect("Failed to prove");
        verifier
            .verify(&proof, &[x, o])
            .expect("Failed to verify the proof");
    }
}
