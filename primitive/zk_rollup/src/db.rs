use std::collections::BTreeMap;

use red_jubjub::PublicKey;

use crate::operator::UserData;

#[derive(Default)]
pub(crate) struct Db {
    users: BTreeMap<PublicKey, UserData>,
}

impl Db {
    pub fn get(&self, k: &PublicKey) -> &UserData {
        self.users
            .get(k)
            .expect("User is not presented in the state")
    }

    pub fn get_mut(&mut self, k: &PublicKey) -> &mut UserData {
        self.users
            .get_mut(k)
            .expect("User is not presented in the state")
    }

    pub fn insert(&mut self, key: PublicKey, value: UserData) {
        self.users.insert(key, value);
    }
}
