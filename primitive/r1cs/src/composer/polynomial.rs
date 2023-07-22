// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use zkstd::common::Pairing;

use crate::constraint_system::Witness;

/// Represents a polynomial in coefficient form with its associated wire data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polynomial<P: Pairing> {
    // Selectors
    /// Multiplier selector
    pub(crate) q_m: P::ScalarField,
    /// Left wire selector
    pub(crate) q_l: P::ScalarField,
    /// Right wire selector
    pub(crate) q_r: P::ScalarField,
    /// Output wire selector
    pub(crate) q_o: P::ScalarField,
    /// Constant wire selector
    pub(crate) q_c: P::ScalarField,
    /// Fourth wire selector
    pub(crate) q_d: P::ScalarField,
    /// Arithmetic wire selector
    pub(crate) q_arith: P::ScalarField,
    /// Range selector
    pub(crate) q_range: P::ScalarField,
    /// Logic selector
    pub(crate) q_logic: P::ScalarField,
    /// Fixed base group addition selector
    pub(crate) q_fixed_group_add: P::ScalarField,
    /// Variable base group addition selector
    pub(crate) q_variable_group_add: P::ScalarField,

    /// Left wire witness.
    pub(crate) w_a: Witness,
    /// Right wire witness.
    pub(crate) w_b: Witness,
    /// Fourth wire witness.
    pub(crate) w_d: Witness,
    /// Output wire witness.
    pub(crate) w_o: Witness,
}
