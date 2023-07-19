// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::constraint_system::Witness;

/// Represents a JubJub point in the circuit
#[derive(Debug, Clone, Copy)]
pub struct WitnessPoint {
    x: Witness,
    y: Witness,
}

impl WitnessPoint {
    #[allow(dead_code)]
    pub(crate) const fn new(x: Witness, y: Witness) -> Self {
        Self { x, y }
    }

    /// Return the X coordinate of the point
    pub const fn x(&self) -> &Witness {
        &self.x
    }

    /// Return the Y coordinate of the point
    pub const fn y(&self) -> &Witness {
        &self.y
    }
}
