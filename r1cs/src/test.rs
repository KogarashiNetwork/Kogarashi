use crate::driver::CircuitDriver;
use crate::matrix::{DenseVectors, SparseMatrix, SparseRow};
use crate::wire::Wire;
use crate::R1cs;

use bn_254::{Fq, Fr, G1Affine};
use zkstd::common::{vec, PrimeField, Vec};

// bn curve b param
pub(crate) const PARAM_B: Fr = Fr::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);
pub const PARAM_B3: Fr = PARAM_B.add_const(PARAM_B).add_const(PARAM_B);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        PARAM_B3
    }
}

fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
    witnesses
        .iter()
        .map(|witness| F::from(*witness))
        .collect::<Vec<_>>()
}

fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>, l: usize) -> SparseMatrix<F> {
    let sparse_matrix = value
        .iter()
        .map(|elements| {
            let coeffs = elements
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    if index <= l {
                        (Wire::Instance(index), F::from(*element))
                    } else {
                        let index = index - l - 1;
                        (Wire::Witness(index), F::from(*element))
                    }
                })
                .filter(|(_, value)| *value != F::zero())
                .collect::<Vec<_>>();
            SparseRow(coeffs)
        })
        .collect::<Vec<_>>();
    SparseMatrix(sparse_matrix)
}

fn example_z_witness<F: PrimeField>(input: u64, l: usize) -> (DenseVectors<F>, DenseVectors<F>) {
    let z = array_to_witnessess(vec![
        1,
        input,
        input * input * input + input + 5,
        input * input,
        input * input * input,
        input * input * input + input,
    ]);
    let (public_inputs, witness) = z.split_at(l + 1);
    (
        DenseVectors::new(public_inputs.to_vec()),
        DenseVectors::new(witness.to_vec()),
    )
}

pub fn example_r1cs<C: CircuitDriver>(input: u64) -> R1cs<C> {
    let m = 4;
    let l = 1;
    let a = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0],
            vec![5, 0, 0, 0, 0, 1],
        ],
        l,
    );
    let b = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ],
        l,
    );
    let c = dense_to_sparse(
        vec![
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 0, 0],
        ],
        l,
    );
    let (x, w) = example_z_witness(input, l);
    R1cs { m, a, b, c, x, w }
}
