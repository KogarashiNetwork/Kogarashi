// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
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

//! Types that should only be used for testing!

use crate::{CryptoStore, Error, SyncCryptoStore, SyncCryptoStorePtr};
use async_trait::async_trait;
use parking_lot::RwLock;
use sp_core::crypto::KeyTypeId;
use sp_core::crypto::{CryptoTypePublicPair, Pair, Public};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// A keystore implementation usable in tests.
#[derive(Default)]
pub struct KeyStore {
    /// `KeyTypeId` maps to public keys and public keys map to private keys.
    keys: Arc<RwLock<HashMap<KeyTypeId, HashMap<Vec<u8>, String>>>>,
}

impl KeyStore {
    /// Creates a new instance of `Self`.
    pub fn new() -> Self {
        Self::default()
    }

    fn redjubjub_key_pair(
        &self,
        id: KeyTypeId,
        pub_key: &redjubjub::Public,
    ) -> Option<redjubjub::Pair> {
        self.keys.read().get(&id).and_then(|inner| {
            inner.get(pub_key.as_slice()).map(|s| {
                redjubjub::Pair::from_string(s, None).expect("`redjubjub` seed slice is valid")
            })
        })
    }
}

#[async_trait]
impl CryptoStore for KeyStore {
    async fn keys(&self, id: KeyTypeId) -> Result<Vec<CryptoTypePublicPair>, Error> {
        SyncCryptoStore::keys(self, id)
    }

    async fn redjubjub_public_keys(&self, id: KeyTypeId) -> Vec<redjubjub::Public> {
        SyncCryptoStore::redjubjub_public_keys(self, id)
    }

    async fn redjubjub_generate_new(
        &self,
        id: KeyTypeId,
        seed: Option<&str>,
    ) -> Result<redjubjub::Public, Error> {
        SyncCryptoStore::redjubjub_generate_new(self, id, seed)
    }

    async fn insert_unknown(&self, id: KeyTypeId, suri: &str, public: &[u8]) -> Result<(), ()> {
        SyncCryptoStore::insert_unknown(self, id, suri, public)
    }

    async fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> bool {
        SyncCryptoStore::has_keys(self, public_keys)
    }

    async fn supported_keys(
        &self,
        id: KeyTypeId,
        keys: Vec<CryptoTypePublicPair>,
    ) -> std::result::Result<Vec<CryptoTypePublicPair>, Error> {
        SyncCryptoStore::supported_keys(self, id, keys)
    }

    async fn sign_with(
        &self,
        id: KeyTypeId,
        key: &CryptoTypePublicPair,
        msg: &[u8],
    ) -> Result<Vec<u8>, Error> {
        SyncCryptoStore::sign_with(self, id, key, msg)
    }
}

impl SyncCryptoStore for KeyStore {
    fn keys(&self, id: KeyTypeId) -> Result<Vec<CryptoTypePublicPair>, Error> {
        self.keys
            .read()
            .get(&id)
            .map(|map| {
                Ok(map.keys().fold(Vec::new(), |mut v, k| {
                    v.push(CryptoTypePublicPair(redjubjub::CRYPTO_ID, k.clone()));
                    v
                }))
            })
            .unwrap_or_else(|| Ok(vec![]))
    }

    fn redjubjub_public_keys(&self, id: KeyTypeId) -> Vec<redjubjub::Public> {
        self.keys
            .read()
            .get(&id)
            .map(|keys| {
                keys.values()
                    .map(|s| {
                        redjubjub::Pair::from_string(s, None)
                            .expect("`redjubjub` seed slice is valid")
                    })
                    .map(|p| p.public())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn redjubjub_generate_new(
        &self,
        id: KeyTypeId,
        seed: Option<&str>,
    ) -> Result<redjubjub::Public, Error> {
        match seed {
            Some(seed) => {
                let pair = redjubjub::Pair::from_string(seed, None).map_err(|_| {
                    Error::ValidationError("Generates an `redjubjub` pair.".to_owned())
                })?;
                self.keys
                    .write()
                    .entry(id)
                    .or_default()
                    .insert(pair.public().to_raw_vec(), seed.into());
                Ok(pair.public())
            }
            None => {
                let (pair, phrase, _) = redjubjub::Pair::generate_with_phrase(None);
                self.keys
                    .write()
                    .entry(id)
                    .or_default()
                    .insert(pair.public().to_raw_vec(), phrase);
                Ok(pair.public())
            }
        }
    }

    fn insert_unknown(&self, id: KeyTypeId, suri: &str, public: &[u8]) -> Result<(), ()> {
        self.keys
            .write()
            .entry(id)
            .or_default()
            .insert(public.to_owned(), suri.to_string());
        Ok(())
    }

    fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> bool {
        public_keys
            .iter()
            .all(|(k, t)| self.keys.read().get(&t).and_then(|s| s.get(k)).is_some())
    }

    fn supported_keys(
        &self,
        id: KeyTypeId,
        keys: Vec<CryptoTypePublicPair>,
    ) -> std::result::Result<Vec<CryptoTypePublicPair>, Error> {
        let provided_keys = keys.into_iter().collect::<HashSet<_>>();
        let all_keys = SyncCryptoStore::keys(self, id)?
            .into_iter()
            .collect::<HashSet<_>>();

        Ok(provided_keys.intersection(&all_keys).cloned().collect())
    }

    fn sign_with(
        &self,
        id: KeyTypeId,
        key: &CryptoTypePublicPair,
        msg: &[u8],
    ) -> Result<Vec<u8>, Error> {
        use codec::Encode;

        match key.0 {
            redjubjub::CRYPTO_ID => {
                let key_pair: redjubjub::Pair = self
                    .redjubjub_key_pair(id, &redjubjub::Public::from_slice(key.1.as_slice()))
                    .ok_or_else(|| Error::PairNotFound("redjubjub".to_owned()))?;
                return Ok(key_pair.sign(msg).encode());
            }
            _ => Err(Error::KeyNotSupported(id)),
        }
    }
}

impl Into<SyncCryptoStorePtr> for KeyStore {
    fn into(self) -> SyncCryptoStorePtr {
        Arc::new(self)
    }
}

impl Into<Arc<dyn CryptoStore>> for KeyStore {
    fn into(self) -> Arc<dyn CryptoStore> {
        Arc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SyncCryptoStore;
    use sp_core::crypto::KeyTypeId;

    pub const REDJUBJUB: KeyTypeId = KeyTypeId(*b"redj");

    #[test]
    fn store_key_and_extract() {
        let store = KeyStore::new();

        let public = SyncCryptoStore::redjubjub_generate_new(&store, REDJUBJUB, None)
            .expect("Generates key");

        let public_keys = SyncCryptoStore::keys(&store, REDJUBJUB).unwrap();

        assert!(public_keys.contains(&public.into()));
    }

    #[test]
    fn store_unknown_and_extract_it() {
        let store = KeyStore::new();

        let secret_uri = "//Alice";
        let key_pair = redjubjub::Pair::from_string(secret_uri, None).expect("Generates key pair");

        SyncCryptoStore::insert_unknown(&store, REDJUBJUB, secret_uri, key_pair.public().as_ref())
            .expect("Inserts unknown key");

        let public_keys = SyncCryptoStore::keys(&store, REDJUBJUB).unwrap();

        assert!(public_keys.contains(&key_pair.public().into()));
    }
}
