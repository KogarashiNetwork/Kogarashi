mod blueprint;
mod matrix;
mod wire;

pub use blueprint::R1csStruct;
pub use matrix::*;
pub use wire::Wire;

use core::ops::Index;
use zkstd::common::PrimeField;

#[derive(Debug)]
pub struct R1cs<F: PrimeField> {
    // r1cs structure
    pub s: R1csStruct<F>,
    // r1cs witness includes private inputs and intermediate value
    w: DenseVectors<F>,
    // r1cs instance includes public inputs and outputs
    x: DenseVectors<F>,
}

impl<F: PrimeField> R1cs<F> {
    pub fn m(&self) -> usize {
        self.s.m()
    }

    pub fn l(&self) -> usize {
        self.x.len()
    }

    pub fn m_l_1(&self) -> usize {
        self.w.len()
    }

    pub fn alloc_instance(&mut self, instance: F) -> Wire {
        let wire = self.public_wire();
        self.x.push(instance);
        wire
    }

    pub fn alloc_witness(&mut self, witness: F) -> Wire {
        let wire = self.private_wire();
        self.w.push(witness);
        wire
    }

    pub fn constrain_mul(&mut self, x: SparseRow<F>, y: SparseRow<F>, z: SparseRow<F>) {
        self.s.append(x, y, z)
    }

    pub fn constrain_add(&mut self, x: SparseRow<F>, y: SparseRow<F>, z: SparseRow<F>) {
        self.s.append(x + y, SparseRow::from(Wire::ONE), z)
    }

    pub fn w(&self) -> DenseVectors<F> {
        self.w.clone()
    }

    pub fn x(&self) -> DenseVectors<F> {
        self.x.clone()
    }

    fn public_wire(&self) -> Wire {
        Wire::Instance(self.x.len())
    }

    fn private_wire(&self) -> Wire {
        Wire::Witness(self.w.len())
    }
}

impl<F: PrimeField> Default for R1cs<F> {
    fn default() -> Self {
        Self {
            s: R1csStruct::default(),
            w: DenseVectors::new(vec![F::one()]),
            x: DenseVectors::default(),
        }
    }
}

impl<F: PrimeField> Index<Wire> for R1cs<F> {
    type Output = F;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Witness(i) => &self.w[i],
            Wire::Instance(i) => &self.x[i],
        }
    }
}