// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::ops::Index;

use zero_crypto::{
    behave::{Curve, Ring},
    common::Pairing,
};

use crate::constraint_system::{Constraint, Witness, WitnessPoint};

pub(crate) mod builder;
pub(crate) mod circuit;
pub(crate) mod compiler;
pub(crate) mod polynomial;
pub(crate) mod prover;
pub(crate) mod verifier;

pub use builder::Builder;
pub use circuit::Circuit;
pub use compiler::Compiler;
pub use polynomial::Polynomial;
pub use prover::Prover;
pub use verifier::Verifier;

pub trait Composer<P: Pairing>: Sized + Index<Witness, Output = P::ScalarField> {
    /// Zero representation inside the constraint system.
    ///
    /// A turbo composer expects the first witness to be always present and to
    /// be zero.
    const ZERO: Witness = Witness::new(0);

    /// `One` representation inside the constraint system.
    ///
    /// A turbo composer expects the 2nd witness to be always present and to
    /// be one.
    const ONE: Witness = Witness::new(1);

    /// Identity point representation inside the constraint system
    const IDENTITY: WitnessPoint = WitnessPoint::new(Self::ZERO, Self::ONE);

    /// Create an empty constraint system.
    ///
    /// This shouldn't be used directly; instead, use [`Self::initialized`]
    fn uninitialized(capacity: usize) -> Self;

    /// Constraints count
    fn constraints(&self) -> usize;

    /// Allocate a witness value into the composer and return its index.
    fn append_witness_internal(&mut self, witness: P::ScalarField) -> Witness;

    /// Append a new width-4 poly gate/constraint.
    fn append_custom_gate_internal(&mut self, constraint: Constraint<P>);

    /// Allocate a witness value into the composer and return its index.
    fn append_witness<W: Into<P::ScalarField>>(&mut self, witness: W) -> Witness {
        let witness = witness.into();

        #[allow(deprecated)]
        let witness = self.append_witness_internal(witness);

        // let v = self[witness];

        witness
    }

    /// Appends a point in affine form as [`WitnessPoint`]
    fn append_point<AP: Into<P::JubjubAffine>>(&mut self, affine: AP) -> WitnessPoint {
        let affine = affine.into();

        let x = self.append_witness(affine.get_x());
        let y = self.append_witness(affine.get_y());

        WitnessPoint::new(x, y)
    }

    /// Constrain a scalar into the circuit description and return an allocated
    /// [`Witness`] with its value
    fn append_constant<C: Into<P::ScalarField>>(&mut self, constant: C) -> Witness {
        let constant = constant.into();
        let witness = self.append_witness(constant);

        self.assert_equal_constant(witness, constant, None);

        witness
    }

    /// Append a new width-4 poly gate/constraint.
    ///
    /// The constraint added will enforce the following:
    /// `q_m · a · b  + q_l · a + q_r · b + q_o · o + q_4 · d + q_c + PI = 0`.
    fn append_gate(&mut self, constraint: Constraint<P>) {
        let constraint = Constraint::arithmetic(&constraint);

        self.append_custom_gate_internal(constraint)
    }

    /// Asserts `a == b` by appending a gate
    fn assert_equal(&mut self, a: Witness, b: Witness) {
        let constraint = Constraint::new()
            .left(1)
            .right(-P::ScalarField::one())
            .a(a)
            .b(b);

        self.append_gate(constraint);
    }

    /// Adds a boolean constraint (also known as binary constraint) where the
    /// gate eq. will enforce that the [`Witness`] received is either `0` or `1`
    /// by adding a constraint in the circuit.
    ///
    /// Note that using this constraint with whatever [`Witness`] that
    /// is not representing a value equalling 0 or 1, will always force the
    /// equation to fail.
    fn component_boolean(&mut self, a: Witness) {
        let zero = Self::ZERO;
        let constraint = Constraint::new()
            .mult(1)
            .output(-P::ScalarField::one())
            .a(a)
            .b(a)
            .o(a)
            .d(zero);

        self.append_gate(constraint);
    }

    /// Constrain `a` to be equal to `constant + pi`.
    ///
    /// `constant` will be defined as part of the public circuit description.
    fn assert_equal_constant<C: Into<P::ScalarField>>(
        &mut self,
        a: Witness,
        constant: C,
        public: Option<P::ScalarField>,
    ) {
        let constant = constant.into();
        let constraint = Constraint::new().left(1).constant(-constant).a(a);
        let constraint = public
            .map(|p| constraint.clone().public(p))
            .unwrap_or(constraint);

        self.append_gate(constraint);
    }

    /// Adds blinding factors to the witness polynomials with two dummy
    /// arithmetic constraints
    fn append_dummy_gates(&mut self) {
        let six = self.append_witness(P::ScalarField::from(6));
        let one = self.append_witness(P::ScalarField::from(1));
        let seven = self.append_witness(P::ScalarField::from(7));
        let min_twenty = self.append_witness(-P::ScalarField::from(20));

        // Add a dummy constraint so that we do not have zero polynomials
        let constraint = Constraint::new()
            .mult(1)
            .left(2)
            .right(3)
            .fourth(1)
            .constant(4)
            .output(4)
            .a(six)
            .b(seven)
            .d(one)
            .o(min_twenty);

        self.append_gate(constraint);

        // Add another dummy constraint so that we do not get the identity
        // permutation
        let constraint = Constraint::new()
            .mult(1)
            .left(1)
            .right(1)
            .constant(127)
            .output(1)
            .a(min_twenty)
            .b(six)
            .o(seven);

        self.append_gate(constraint);
    }

    /// Initialize the constraint system with dummy gates
    fn initialized(capacity: usize) -> Self {
        #[allow(deprecated)]
        let mut slf = Self::uninitialized(capacity);

        let zero = slf.append_witness(0);
        let one = slf.append_witness(1);

        slf.assert_equal_constant(zero, 0, None);
        slf.assert_equal_constant(one, 1, None);

        slf.append_dummy_gates();
        slf.append_dummy_gates();

        slf
    }
}
