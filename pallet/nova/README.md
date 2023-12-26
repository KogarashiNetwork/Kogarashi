# Nova Module

The Nova module provides functionality for Recursive Snarks.

## Overview

The Nova module provides functions for:

- Defining circuit to be folded
- Folding custom circuit r1cs
- Setup public parameters
- Checking satisfiability of folded r1cs

## Terminology

- **Recursive Snark**: One types of Snarks which proves a proof inside of another proof. [halo2](https://github.com/zcash/halo2) accumulation scheme is one of the most famous recursive Snark example. Prover proves the validity of verifier in addition to prover arithmetization.

- **Cycle of Curves**: Curve pair which has inverse scalar and base field for each other. Prover arithmetization is performed on base field and verifier arithmetization is on scalar field. In recursive snark, prover proves both arithmetization thus, cycle of curves can be used for optimization. [pasta](https://github.com/zcash/pasta_curves) curves is one of the most famous cycle of curves example.

- **Folding Scheme**: One types of recursive snark strategy. Folding scheme compresses the statement instead of generating proof. We can skip heavy computation such as the Fft and large Msm by avoiding proof generation.

- **IVC Scheme** One types of [Proof-Carrying Data](https://eprint.iacr.org/2020/1618.pdf). IVC (Incrementally Verifiable Computation) means computations over path graphs.

## Interface

### Dispatchable Functions

- `verify` - Verify IVC scheme final proof

## Usage

The following examples show how to use the Nova module in your custom module.

### Examples from the FRAME

The Nova module uses the `FunctionCircuit` trait to define circuit to be folded:

```rs
impl<F: PrimeField> FunctionCircuit<F> for ExampleFunction<F> {
    fn invoke(z: &DenseVectors<F>) -> DenseVectors<F> {
        let next_z = z[0] * z[0] * z[0] + z[0] + F::from(5);
        DenseVectors::new(vec![next_z])
    }

    fn invoke_cs<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        z_i: Vec<FieldAssignment<F>>,
    ) -> Vec<FieldAssignment<F>> {
        let five = FieldAssignment::constant(&F::from(5));
        let z_i_square = FieldAssignment::mul(cs, &z_i[0], &z_i[0]);
        let z_i_cube = FieldAssignment::mul(cs, &z_i_square, &z_i[0]);

        vec![&(&z_i_cube + &z_i[0]) + &five]
    }
}
```

- `invoke` - Return function result
- `invoke_cs` - Custom circuit constraints

In above example, we prove $f(x) = x^3 + x + 5$ for given input $x$.
The Nova module verifies the folding scheme validity by `verify` pallet function.

```rs
let z0_primary = DenseVectors::new(vec![Fr::from(0)]);
let z0_secondary = DenseVectors::new(vec![Fq::from(0)]);
let mut ivc =
    Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>, ExampleFunction<Fq>>::init(
        &pp,
        z0_primary,
        z0_secondary,
    );
(0..2).for_each(|_| {
    ivc.prove_step(&pp);
});
let proof = ivc.prove_step(&pp);

new_test_ext().execute_with(|| {
    assert!(Nova::verify(Origin::signed(1), proof, pp.clone()).is_ok());
});
```

In above example, we verify the validity of folding $x_3 = f^{(3)}(x)$

## Test

```shell
$ cargo test --all --release
```

License: Apache-2.0
