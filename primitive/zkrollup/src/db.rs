use sp_std::collections::btree_map::BTreeMap;

use red_jubjub::PublicKey;
use zkstd::common::Pairing;

use crate::domain::UserData;

#[derive(Default)]
pub(crate) struct Db<P: Pairing> {
    users: BTreeMap<PublicKey<P>, UserData<P>>,
}

impl<P: Pairing> Db<P> {
    pub fn get(&self, k: &PublicKey<P>) -> &UserData<P> {
        self.users
            .get(k)
            .expect("User is not presented in the state")
    }

    pub fn get_mut(&mut self, k: &PublicKey<P>) -> &mut UserData<P> {
        self.users
            .get_mut(k)
            .expect("User is not presented in the state")
    }

    pub fn insert(&mut self, key: PublicKey<P>, value: UserData<P>) {
        self.users.insert(key, value);
    }
}
