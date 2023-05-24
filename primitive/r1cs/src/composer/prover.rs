// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::marker::PhantomData;

use zero_crypto::common::Pairing;

use super::Circuit;

/// Turbo Prover with processed keys
#[derive(Clone)]
pub struct Prover<C, P>
where
    C: Circuit<P>,
    P: Pairing,
{
    circuit: PhantomData<C>,
    pairing: PhantomData<P>,
}
