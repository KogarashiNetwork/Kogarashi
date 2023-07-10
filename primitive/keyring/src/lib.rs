use sp_runtime::AccountId32;
use zero_jubjub::redsa::PublicKey;

/// Set of test accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Keyring {
    Alice,
    Bob,
    Charlie,
    Dave,
    Eve,
    Ferdie,
    One,
    Two,
}

impl Keyring {
    pub fn from_public(who: &Public) -> Option<Keyring> {
        Self::iter().find(|&k| &Public::from(k) == who)
    }

    pub fn from_account_id(who: &AccountId32) -> Option<Keyring> {
        Self::iter().find(|&k| &k.to_account_id() == who)
    }

    pub fn from_raw_public(who: [u8; 32]) -> Option<Keyring> {
        Self::from_public(&Public::from_raw(who))
    }

    pub fn to_raw_public(self) -> [u8; 32] {
        *Public::from(self).as_array_ref()
    }

    pub fn from_h256_public(who: H256) -> Option<Keyring> {
        Self::from_public(&Public::from_raw(who.into()))
    }

    pub fn to_h256_public(self) -> H256 {
        Public::from(self).as_array_ref().into()
    }

    pub fn to_raw_public_vec(self) -> Vec<u8> {
        Public::from(self).to_raw_vec()
    }

    pub fn to_account_id(self) -> AccountId32 {
        self.to_raw_public().into()
    }

    pub fn sign(self, msg: &[u8]) -> Signature {
        Pair::from(self).sign(msg)
    }

    pub fn pair(self) -> Pair {
        Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
            .expect("static values are known good; qed")
    }

    /// Returns an iterator over all test accounts.
    pub fn iter() -> impl Iterator<Item = Keyring> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn public(self) -> Public {
        self.pair().public()
    }

    pub fn to_seed(self) -> String {
        format!("//{}", self)
    }
}
