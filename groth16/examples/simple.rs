use bn_254::Fr as BnScalar;
use zkgroth16::{Bn254Driver, Circuit, Error, ZkSnark};
use zkstd::{
    circuit::prelude::{FieldAssignment, R1cs},
    common::OsRng,
};

// Circuit definition
// x: public input
// o: public output
// Constraints are as follows
// x^3 + x + 5 = o
#[derive(Debug)]
pub struct DummyCircuit {
    x: BnScalar,
    o: BnScalar,
}

impl DummyCircuit {
    pub fn new(x: BnScalar, o: BnScalar) -> Self {
        Self { x, o }
    }
}

impl Default for DummyCircuit {
    fn default() -> Self {
        Self::new(0.into(), 0.into())
    }
}

impl Circuit for DummyCircuit {
    fn synthesize(&self, composer: &mut R1cs<Bn254Driver>) -> Result<(), Error> {
        // Declare public input
        let x = FieldAssignment::instance(composer, self.x);
        // Declare public output
        let o = FieldAssignment::instance(composer, self.o);
        // Declare public constant
        let c = FieldAssignment::constant(&BnScalar::from(5));

        // Constrain sym1 == x * x
        let sym1 = FieldAssignment::mul(composer, &x, &x);
        // Constrain y == sym1 * x
        let y = FieldAssignment::mul(composer, &sym1, &x);
        // Constrain sym2 = y + x
        let sym2 = FieldAssignment::add(composer, &y, &x);

        // Constrain sym2 + c == o
        FieldAssignment::enforce_eq(composer, &(&sym2 + &c), &o);

        Ok(())
    }
}

fn main() {
    // Public input and output
    let x = BnScalar::from(3);
    let o = BnScalar::from(35);

    // Initialize circuit with arguments
    let circuit = DummyCircuit::new(x, o);

    // Setup prover and verifier
    let (mut prover, verifier) =
        ZkSnark::setup::<DummyCircuit>(OsRng).expect("Failed to compile circuit");

    // Generate proof
    let proof = prover
        .create_proof(&mut OsRng, circuit)
        .expect("Failed to prove");

    // Verify proof
    verifier
        .verify(&proof, &[x, o])
        .expect("Failed to verify the proof");
}
