use super::binary::BinaryAssignment;
use super::field::FieldAssignment;

use crate::circuit::CircuitDriver;
use crate::common::{BNAffine, BNProjective, CurveGroup, Group, IntGroup, Ring};
use crate::r1cs::R1cs;

#[derive(Clone)]
pub struct PointAssignment<C: CircuitDriver> {
    x: FieldAssignment<C>,
    y: FieldAssignment<C>,
    z: FieldAssignment<C>,
}

impl<C: CircuitDriver> PointAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, point: C::Affine) -> Self {
        let x = FieldAssignment::instance(cs, point.get_x().into());
        let y = FieldAssignment::instance(cs, point.get_y().into());
        let z = FieldAssignment::instance(
            cs,
            if point.is_identity() {
                C::Scalar::zero()
            } else {
                C::Scalar::one()
            },
        );

        Self { x, y, z }
    }

    pub fn witness(cs: &mut R1cs<C>, x: C::Scalar, y: C::Scalar, is_infinity: bool) -> Self {
        let x = FieldAssignment::witness(cs, x);
        let y = FieldAssignment::witness(cs, y);
        let z = FieldAssignment::witness(
            cs,
            if is_infinity {
                C::Scalar::zero()
            } else {
                C::Scalar::one()
            },
        );

        Self { x, y, z }
    }

    pub fn assert_equal_public_point(
        &self,
        cs: &mut R1cs<C>,
        point: <C::Affine as BNAffine>::Extended,
    ) {
        let point_x = FieldAssignment::constant(&point.get_x().into());
        let point_y = FieldAssignment::constant(&point.get_y().into());
        let point_z = FieldAssignment::constant(&point.get_z().into());

        let xz1 = FieldAssignment::mul(cs, &self.x, &point_z);
        let xz2 = FieldAssignment::mul(cs, &point_x, &self.z);

        FieldAssignment::enforce_eq(cs, &xz1, &xz2);

        let yz1 = FieldAssignment::mul(cs, &self.y, &point_z);
        let yz2 = FieldAssignment::mul(cs, &point_y, &self.z);

        FieldAssignment::enforce_eq(cs, &yz1, &yz2);
    }

    pub fn add(&self, rhs: &Self, cs: &mut R1cs<C>) -> Self {
        let b3 = FieldAssignment::<C>::constant(&C::b3().into());
        let t0 = FieldAssignment::mul(cs, &self.x, &rhs.x);
        let t1 = FieldAssignment::mul(cs, &self.y, &rhs.y);
        let t2 = FieldAssignment::mul(cs, &self.z, &rhs.z);
        let t3 = &self.x + &self.y;
        let t4 = &rhs.x + &rhs.y;
        let t3 = FieldAssignment::mul(cs, &t3, &t4);
        let t4 = &t0 + &t1;
        let t3 = &t3 - &t4;
        let t4 = &self.y + &self.z;
        let x3 = &rhs.y + &rhs.z;
        let t4 = FieldAssignment::mul(cs, &t4, &x3);
        let x3 = &t1 + &t2;
        let t4 = &t4 - &x3;
        let x3 = &self.x + &self.z;
        let y3 = &rhs.x + &rhs.z;
        let x3 = FieldAssignment::mul(cs, &x3, &y3);
        let y3 = &t0 + &t2;
        let y3 = &x3 - &y3;
        let x3 = &t0 + &t0;
        let t0 = &x3 + &t0;
        let t2 = FieldAssignment::mul(cs, &t2, &b3);
        let z3 = &t1 + &t2;
        let t1 = &t1 - &t2;
        let y3 = FieldAssignment::mul(cs, &y3, &b3);
        let x3 = FieldAssignment::mul(cs, &t4, &y3);
        let t2 = FieldAssignment::mul(cs, &t3, &t1);
        let x3 = &t2 - &x3;
        let y3 = FieldAssignment::mul(cs, &y3, &t0);
        let t1 = FieldAssignment::mul(cs, &t1, &z3);
        let y3 = &t1 + &y3;
        let t0 = FieldAssignment::mul(cs, &t0, &t3);
        let z3 = FieldAssignment::mul(cs, &z3, &t4);
        let z3 = &z3 + &t0;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    pub fn double(&self, cs: &mut R1cs<C>) -> Self {
        let b3 = FieldAssignment::<C>::constant(&C::b3().into());
        let t0 = FieldAssignment::mul(cs, &self.y, &self.y);
        let z3 = &t0 + &t0;
        let z3 = &z3 + &z3;
        let z3 = &z3 + &z3;
        let t1 = FieldAssignment::mul(cs, &self.y, &self.z);
        let t2 = FieldAssignment::mul(cs, &self.z, &self.z);
        let t2 = FieldAssignment::mul(cs, &t2, &b3);
        let x3 = FieldAssignment::mul(cs, &t2, &z3);
        let y3 = &t0 + &t2;
        let z3 = FieldAssignment::mul(cs, &t1, &z3);
        let t1 = &t2 + &t2;
        let t2 = &t1 + &t2;
        let t0 = &t0 - &t2;
        let y3 = FieldAssignment::mul(cs, &t0, &y3);
        let y3 = &x3 + &y3;
        let t1 = FieldAssignment::mul(cs, &self.x, &self.y);
        let x3 = FieldAssignment::mul(cs, &t0, &t1);
        let x3 = &x3 + &x3;

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// coordinate scalar
    pub fn scalar_point(&self, cs: &mut R1cs<C>, scalar: &FieldAssignment<C>) -> Self {
        let i = C::Affine::ADDITIVE_IDENTITY;
        let mut res = PointAssignment::instance(cs, i);
        for bit in FieldAssignment::to_bits(cs, scalar).iter() {
            res = res.double(cs);
            let point_to_add = self.select_identity(cs, bit);
            res = res.add(&point_to_add, cs);
        }

        res
    }

    pub fn select_identity(&self, cs: &mut R1cs<C>, bit: &BinaryAssignment<C>) -> Self {
        let PointAssignment { x, y, z } = self.clone();
        let bit = FieldAssignment::from(bit);
        Self {
            x: FieldAssignment::mul(cs, &x, &bit),
            y: &(&FieldAssignment::mul(cs, &y, &bit)
                + &FieldAssignment::constant(&C::Scalar::one()))
                - &bit,
            z: FieldAssignment::mul(cs, &z, &bit),
        }
    }

    pub fn get_x(&self) -> FieldAssignment<C> {
        self.x.clone()
    }

    pub fn get_y(&self) -> FieldAssignment<C> {
        self.y.clone()
    }

    pub fn get_z(&self) -> FieldAssignment<C> {
        self.z.clone()
    }
}
