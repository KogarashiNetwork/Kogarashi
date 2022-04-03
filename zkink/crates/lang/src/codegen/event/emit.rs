// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::reflect::ContractEventBase;

/// Allows for `self.env().emit_event(...)` syntax in ink! implementation blocks.
pub trait EmitEvent<C>
where
    C: ContractEventBase,
{
    /// Emits an event that can be trivially converted into the base event.
    fn emit_event<E>(self, event: E)
    where
        E: Into<<C as ContractEventBase>::Type>;
}
