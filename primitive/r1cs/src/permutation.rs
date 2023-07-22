// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::constraint_system::{WireData, Witness};

use core::marker::PhantomData;
use hashbrown::HashMap;

use zkstd::behave::*;

/// Permutation provides the necessary state information and functions
/// to create the permutation polynomial. In the literature, Z(X) is the
/// "accumulator", this is what this codebase calls the permutation polynomial.
#[derive(Debug, Clone)]
pub(crate) struct Permutation<P: Pairing> {
    // Maps a witness to the wires that it is associated to.
    pub(crate) witness_map: HashMap<Witness, Vec<WireData>>,
    _marker: PhantomData<P>,
}

impl<P: Pairing> Permutation<P> {
    /// Creates a Permutation struct with an expected capacity of zero.
    pub(crate) fn new() -> Permutation<P> {
        Permutation::with_capacity(0)
    }

    /// Creates a Permutation struct with an expected capacity of `n`.
    pub(crate) fn with_capacity(size: usize) -> Permutation<P> {
        Permutation {
            witness_map: HashMap::with_capacity(size),
            _marker: PhantomData,
        }
    }

    /// Creates a new [`Witness`] by incrementing the index of the
    /// `witness_map`.
    ///
    /// This is correct as whenever we add a new [`Witness`] into the system It
    /// is always allocated in the `witness_map`.
    pub(crate) fn new_witness(&mut self) -> Witness {
        // Generate the Witness
        let var = Witness::new(self.witness_map.keys().len());

        // Allocate space for the Witness on the witness_map
        // Each vector is initialized with a capacity of 16.
        // This number is a best guess estimate.
        self.witness_map.insert(var, Vec::with_capacity(16usize));

        var
    }

    /// Checks that the [`Witness`]s are valid by determining if they have been
    /// added to the system
    fn valid_witnesses(&self, witnesses: &[Witness]) -> bool {
        witnesses
            .iter()
            .all(|var| self.witness_map.contains_key(var))
    }

    /// Maps a set of [`Witness`]s (a,b,c,d) to a set of [`Wire`](WireData)s
    /// (left, right, out, fourth) with the corresponding gate index
    pub fn add_witnesses_to_map<T: Into<Witness>>(
        &mut self,
        a: T,
        b: T,
        c: T,
        d: T,
        gate_index: usize,
    ) {
        let left: WireData = WireData::Left(gate_index);
        let right: WireData = WireData::Right(gate_index);
        let output: WireData = WireData::Output(gate_index);
        let fourth: WireData = WireData::Fourth(gate_index);

        // Map each witness to the wire it is associated with
        // This essentially tells us that:
        self.add_witness_to_map(a.into(), left);
        self.add_witness_to_map(b.into(), right);
        self.add_witness_to_map(c.into(), output);
        self.add_witness_to_map(d.into(), fourth);
    }

    pub(crate) fn add_witness_to_map<T: Into<Witness> + Copy>(
        &mut self,
        var: T,
        wire_data: WireData,
    ) {
        assert!(self.valid_witnesses(&[var.into()]));

        // Since we always allocate space for the Vec of WireData when a
        // Witness is added to the witness_map, this should never fail
        let vec_wire_data = self.witness_map.get_mut(&var.into()).unwrap();
        vec_wire_data.push(wire_data);
    }
}
