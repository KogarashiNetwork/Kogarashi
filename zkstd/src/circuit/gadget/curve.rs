use super::binary::BinaryAssignment;
use super::field::FieldAssignment;

use crate::circuit::CircuitDriver;
use crate::common::{BNAffine, BNProjective, PrimeField};
use crate::r1cs::R1cs;

#[derive(Clone)]
pub struct PointAssignment<F: PrimeField> {
    x: FieldAssignment<F>,
    y: FieldAssignment<F>,
    z: FieldAssignment<F>,
}

impl<F: PrimeField> PointAssignment<F> {
    pub fn instance<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        point: impl BNAffine<Base = F>,
    ) -> Self {
        let x = FieldAssignment::instance(cs, point.get_x());
        let y = FieldAssignment::instance(cs, point.get_y());
        let z = FieldAssignment::instance(
            cs,
            if point.is_identity() {
                F::zero()
            } else {
                F::one()
            },
        );

        Self { x, y, z }
    }

    pub fn descale<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>) -> Self {
        let take_value =
            FieldAssignment::is_neq(cs, &self.z, &FieldAssignment::constant(&F::zero()));
        let inv = FieldAssignment::witness(cs, self.z.value(cs).invert().unwrap_or_else(F::zero));

        let p = Self {
            x: FieldAssignment::mul(cs, &self.x, &inv),
            y: FieldAssignment::mul(cs, &self.y, &inv),
            z: FieldAssignment::constant(&F::one()),
        };

        p.select_identity(cs, &take_value)
    }

    pub fn identity() -> Self {
        let x = FieldAssignment::constant(&F::zero());
        let y = FieldAssignment::constant(&F::one());
        let z = FieldAssignment::constant(&F::zero());

        Self { x, y, z }
    }

    pub fn witness<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        x: F,
        y: F,
        is_infinity: bool,
    ) -> Self {
        let x = FieldAssignment::witness(cs, x);
        let y = FieldAssignment::witness(cs, y);
        let z = FieldAssignment::witness(cs, if is_infinity { F::zero() } else { F::one() });

        Self { x, y, z }
    }

    pub fn assert_equal_public_point<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        point: impl BNProjective<Base = F>,
    ) {
        let point_x = FieldAssignment::constant(&point.get_x());
        let point_y = FieldAssignment::constant(&point.get_y());
        let point_z = FieldAssignment::constant(&point.get_z());

        let xz1 = FieldAssignment::mul(cs, &self.x, &point_z);
        let xz2 = FieldAssignment::mul(cs, &point_x, &self.z);

        FieldAssignment::enforce_eq(cs, &xz1, &xz2);

        let yz1 = FieldAssignment::mul(cs, &self.y, &point_z);
        let yz2 = FieldAssignment::mul(cs, &point_y, &self.z);

        FieldAssignment::enforce_eq(cs, &yz1, &yz2);
    }

    pub fn add<C: CircuitDriver<Scalar = F>>(&self, rhs: &Self, cs: &mut R1cs<C>) -> Self {
        let b3 = FieldAssignment::constant(&C::b3());
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

    pub fn double<C: CircuitDriver<Scalar = F>>(&self, cs: &mut R1cs<C>) -> Self {
        let b3 = FieldAssignment::constant(&C::b3());
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
    pub fn scalar_point<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        scalar: &FieldAssignment<F>,
    ) -> Self {
        let mut res = PointAssignment::identity();
        for bit in FieldAssignment::to_bits(cs, scalar, 256).iter() {
            res = res.double(cs);
            let point_to_add = self.select_identity(cs, bit);
            res = res.add(&point_to_add, cs);
        }

        res
    }

    pub fn conditional_select<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        a: &Self,
        b: &Self,
        condition: &BinaryAssignment,
    ) -> PointAssignment<F> {
        let x = FieldAssignment::conditional_select(cs, &a.x, &b.x, condition);
        let y = FieldAssignment::conditional_select(cs, &a.y, &b.y, condition);
        let z = FieldAssignment::conditional_select(cs, &a.z, &b.z, condition);

        Self { x, y, z }
    }

    pub fn select_identity<C: CircuitDriver<Scalar = F>>(
        &self,
        cs: &mut R1cs<C>,
        bit: &BinaryAssignment,
    ) -> Self {
        let PointAssignment { x, y, z } = self.clone();
        let bit = FieldAssignment::from(bit);
        Self {
            x: FieldAssignment::mul(cs, &x, &bit),
            y: &(&FieldAssignment::mul(cs, &y, &bit) + &FieldAssignment::constant(&F::one()))
                - &bit,
            z: FieldAssignment::mul(cs, &z, &bit),
        }
    }

    pub fn get_x(&self) -> FieldAssignment<F> {
        self.x.clone()
    }

    pub fn get_y(&self) -> FieldAssignment<F> {
        self.y.clone()
    }

    pub fn get_z(&self) -> FieldAssignment<F> {
        self.z.clone()
    }
}
