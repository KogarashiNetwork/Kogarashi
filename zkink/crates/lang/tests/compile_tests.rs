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

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/blake2b/pass/*.rs");
    t.compile_fail("tests/ui/blake2b/fail/*.rs");

    t.pass("tests/ui/selector_id/pass/*.rs");
    t.compile_fail("tests/ui/selector_id/fail/*.rs");

    t.pass("tests/ui/selector_bytes/pass/*.rs");
    t.compile_fail("tests/ui/selector_bytes/fail/*.rs");

    t.pass("tests/ui/contract/pass/*.rs");
    t.compile_fail("tests/ui/contract/fail/*.rs");

    t.pass("tests/ui/trait_def/pass/*.rs");
    t.compile_fail("tests/ui/trait_def/fail/*.rs");

    t.pass("tests/ui/chain_extension/E-01-simple.rs");
}
