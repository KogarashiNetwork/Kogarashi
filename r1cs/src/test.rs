use crate::circuit::CircuitDriver;
use crate::matrix::{DenseVectors, SparseMatrix, SparseRow};
use crate::wire::Wire;
use crate::R1cs;

use zkstd::common::{vec, PrimeField, Vec};

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

pub(crate) fn example_r1cs<C: CircuitDriver>(input: u64) -> R1cs<C> {
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
