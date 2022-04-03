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

use ink_lang_codegen::generate_code;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

pub fn analyze(config: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match analyze_or_err(config, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

pub fn analyze_or_err(config: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let trait_definition = ink_lang_ir::InkTraitDefinition::new(config, input)?;
    Ok(generate_code(&trait_definition))
}
