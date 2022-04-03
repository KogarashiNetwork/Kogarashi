use dusk_jubjub::GENERATOR_EXTENDED;
use dusk_plonk::prelude::*;
use rand_core::OsRng;

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input

#[derive(Debug, Default)]
pub struct TestCircuit {
    pub a: BlsScalar,
    pub b: BlsScalar,
    pub c: BlsScalar,
    pub d: BlsScalar,
    pub e: JubJubScalar,
    pub f: JubJubAffine,
}

impl Circuit for TestCircuit {
    const CIRCUIT_ID: [u8; 32] = [0xff; 32];
    fn gadget(&mut self, composer: &mut TurboComposer) -> Result<(), Error> {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint = Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 1 << 6);
        composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint = Constraint::new()
            .mult(1)
            .output(1)
            .public(-self.d)
            .a(a)
            .b(b);

        composer.append_gate(constraint);

        let e = composer.append_witness(self.e);
        let scalar_mul_result = composer.component_mul_generator(e, GENERATOR_EXTENDED);
        composer.assert_equal_public_point(scalar_mul_result, self.f);
        Ok(())
    }

    fn public_inputs(&self) -> Vec<PublicInputValue> {
        vec![self.c.into(), self.d.into(), self.f.into()]
    }

    fn padded_gates(&self) -> usize {
        1 << 11
    }
}

fn main() {
    // Now let's use the Circuit we've just implemented!

    let pp = PublicParameters::setup(1 << 12, &mut OsRng).unwrap();
    // Initialize the circuit
    let mut circuit = TestCircuit::default();
    // Compile the circuit
    let (pk, vd) = circuit.compile(&pp).unwrap();
    // Prover POV
    let proof = {
        let mut circuit = TestCircuit {
            a: BlsScalar::from(20u64),
            b: BlsScalar::from(5u64),
            c: BlsScalar::from(25u64),
            d: BlsScalar::from(100u64),
            e: JubJubScalar::from(2u64),
            f: JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)),
        };
        circuit.prove(&pp, &pk, b"Test").unwrap()
    };
    // Verifier POV
    let public_inputs: Vec<PublicInputValue> = vec![
        BlsScalar::from(25u64).into(),
        BlsScalar::from(100u64).into(),
        JubJubAffine::from(dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64)).into(),
    ];
    TestCircuit::verify(&pp, &vd, &proof, &public_inputs, b"Test").unwrap();
}
