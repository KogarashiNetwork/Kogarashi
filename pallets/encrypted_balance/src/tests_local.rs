// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities

#![cfg(test)]

use crate::{self as pallet_balances, decl_tests, Config};
use frame_support::parameter_types;
use frame_support::traits::StorageMapShim;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};
use zero_elgamal::EncryptedNumber;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>} = 0,
        EncryptedBalances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>} = 1,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
    pub static ExistentialDeposit: u64 = 0;
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
}
impl Config for Test {
    type EncryptedBalance = EncryptedNumber;
    type Event = Event;
    type AccountStore = StorageMapShim<
        super::Account<Test>,
        frame_system::Provider<Test>,
        u64,
        super::AccountData<Self::EncryptedBalance>,
    >;
    type WeightInfo = ();
}
#[derive(Default)]
pub struct ExtBuilder {}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let pk1 = Fr::to_mont_form([1, 0, 0, 0]);
        let pk2 = Fr::to_mont_form([2, 0, 0, 0]);
        let rand1 = Fr::to_mont_form([1, 2, 3, 4]);
        let rand2 = Fr::to_mont_form([4, 3, 2, 1]);
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (1, EncryptedNumber::encrypt(pk1, 50, rand1)),
                (2, EncryptedNumber::encrypt(pk2, 0, rand2)),
            ],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

fn events() -> Vec<Event> {
    let evt = System::events()
        .into_iter()
        .map(|evt| evt.event)
        .collect::<Vec<_>>();

    System::reset_events();

    evt
}

decl_tests! { Test, ExtBuilder }

#[test]
fn emit_events() {
    let pk = Fr::to_mont_form([3, 0, 0, 0]);
    let randomness = Fr::to_mont_form([4, 3, 5, 6]);
    let balance = EncryptedNumber::encrypt(pk, 42, randomness);
    <ExtBuilder>::default().build().execute_with(|| {
        assert_ok!(EncryptedBalances::set_balance(
            RawOrigin::Root.into(),
            3,
            balance
        ));

        assert_eq!(
            events(),
            [
                Event::frame_system(system::Event::NewAccount(3)),
                Event::pallet_balances(crate::Event::Endowed(3, balance)),
                Event::pallet_balances(crate::Event::BalanceSet(3, balance))
            ]
        );
    });
}
