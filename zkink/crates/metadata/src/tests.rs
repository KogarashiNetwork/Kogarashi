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

use super::*;
use pretty_assertions::assert_eq;
use scale_info::{
    IntoPortable,
    Registry,
};
use serde_json::json;

#[test]
fn spec_constructor_selector_must_serialize_to_hex() {
    // given
    let label = "foo";
    let cs = ConstructorSpec::from_label(label)
        .selector(123_456_789u32.to_be_bytes())
        .payable(true)
        .done();
    let mut registry = Registry::new();
    let portable_spec = cs.into_portable(&mut registry);

    // when
    let json = serde_json::to_value(&portable_spec).unwrap();
    let deserialized: ConstructorSpec<PortableForm> =
        serde_json::from_value(json.clone()).unwrap();

    // then
    assert_eq!(
        json,
        json!({
            "label": "foo",
            "payable": true,
            "selector": "0x075bcd15",
            "args": [],
            "docs": []
        })
    );
    assert_eq!(deserialized.selector, portable_spec.selector);
}

#[test]
fn spec_contract_json() {
    // given
    let contract: ContractSpec = ContractSpec::new()
        .constructors(vec![
            ConstructorSpec::from_label("new")
                .selector([94u8, 189u8, 136u8, 214u8])
                .payable(true)
                .args(vec![MessageParamSpec::new("init_value")
                    .of_type(TypeSpec::with_name_segs::<i32, _>(
                        vec!["i32"].into_iter().map(AsRef::as_ref),
                    ))
                    .done()])
                .docs(Vec::new())
                .done(),
            ConstructorSpec::from_label("default")
                .selector([2u8, 34u8, 255u8, 24u8])
                .payable(Default::default())
                .args(Vec::new())
                .docs(Vec::new())
                .done(),
        ])
        .messages(vec![
            MessageSpec::from_label("inc")
                .selector([231u8, 208u8, 89u8, 15u8])
                .mutates(true)
                .payable(true)
                .args(vec![MessageParamSpec::new("by")
                    .of_type(TypeSpec::with_name_segs::<i32, _>(
                        vec!["i32"].into_iter().map(AsRef::as_ref),
                    ))
                    .done()])
                .docs(Vec::new())
                .returns(ReturnTypeSpec::new(None))
                .done(),
            MessageSpec::from_label("get")
                .selector([37u8, 68u8, 74u8, 254u8])
                .mutates(false)
                .payable(false)
                .args(Vec::new())
                .docs(Vec::new())
                .returns(ReturnTypeSpec::new(TypeSpec::with_name_segs::<i32, _>(
                    vec!["i32"].into_iter().map(AsRef::as_ref),
                )))
                .done(),
        ])
        .events(Vec::new())
        .docs(Vec::new())
        .done();

    let mut registry = Registry::new();

    // when
    let json = serde_json::to_value(&contract.into_portable(&mut registry)).unwrap();

    // then
    assert_eq!(
        json,
        json!({
            "constructors": [
                {
                    "args": [
                        {
                            "label": "init_value",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 0
                            }
                        }
                    ],
                    "docs": [],
                    "label": "new",
                    "payable": true,
                    "selector": "0x5ebd88d6"
                },
                {
                    "args": [],
                    "docs": [],
                    "label": "default",
                    "payable": false,
                    "selector": "0x0222ff18"
                }
            ],
            "docs": [],
            "events": [],
            "messages": [
                {
                    "args": [
                        {
                            "label": "by",
                            "type": {
                                "displayName": [
                                    "i32"
                                ],
                                "type": 0
                            }
                        }
                    ],
                    "docs": [],
                    "mutates": true,
                    "payable": true,
                    "label": "inc",
                    "returnType": null,
                    "selector": "0xe7d0590f"
                },
                {
                    "args": [],
                    "docs": [],
                    "mutates": false,
                    "payable": false,
                    "label": "get",
                    "returnType": {
                        "displayName": [
                            "i32"
                        ],
                        "type": 0
                    },
                    "selector": "0x25444afe"
                }
            ],
        })
    )
}

#[test]
fn trim_docs() {
    // given
    let label = "foo";
    let cs = ConstructorSpec::from_label(label)
        .selector(123_456_789u32.to_be_bytes())
        .docs(vec![" foobar      "])
        .payable(Default::default())
        .done();
    let mut registry = Registry::new();
    let compact_spec = cs.into_portable(&mut registry);

    // when
    let json = serde_json::to_value(&compact_spec).unwrap();
    let deserialized: ConstructorSpec<PortableForm> =
        serde_json::from_value(json.clone()).unwrap();

    // then
    assert_eq!(
        json,
        json!({
            "label": "foo",
            "payable": false,
            "selector": "0x075bcd15",
            "args": [],
            "docs": ["foobar"]
        })
    );
    assert_eq!(deserialized.docs, compact_spec.docs);
}
