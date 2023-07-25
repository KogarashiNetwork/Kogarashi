// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use poly_commit::KeyPair;
use zkstd::common::Pairing;

use super::{Builder, Circuit, Composer};
use crate::error::Error;

/// Generate the arguments to prove and verify a circuit
pub struct Compiler;

type CompilerResult<P> = Result<Builder<P>, Error>;

impl Compiler {
    /// Create a new arguments set from a given circuit instance
    ///
    /// Use the default implementation of the circuit
    pub fn compile<C, P>(keypair: &mut KeyPair<P>, label: &[u8]) -> CompilerResult<P>
    where
        C: Circuit<P>,
        P: Pairing,
    {
        Self::compile_with_circuit::<C, P>(keypair, label, &Default::default())
    }

    /// Create a new arguments set from a given circuit instance
    ///
    /// Use the provided circuit instead of the default implementation
    pub fn compile_with_circuit<C, P>(
        keypair: &mut KeyPair<P>,
        _label: &[u8],
        circuit: &C,
    ) -> CompilerResult<P>
    where
        C: Circuit<P>,
        P: Pairing,
    {
        let max_size = (keypair.commit_key().len() - 1) >> 1;
        let mut builder = Builder::initialized(max_size);

        circuit.circuit(&mut builder)?;

        Ok(builder)
    }
}
