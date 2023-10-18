mod transaction;
mod user;

use rand_core::RngCore;
use red_jubjub::{PublicKey, SecretKey, Signature};
use zkstd::common::{FftField, SigUtils};

pub(crate) use transaction::RollupTransactionInfo;
pub use transaction::{Transaction, TransactionData};
pub(crate) use user::UserData;

#[cfg(test)]
mod tests {
    use super::*;
    use jub_jub::{Fp, JubjubExtended};
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::RedJubjub;
    use zkstd::common::{Group, TwistedEdwardsCurve};

    #[test]
    fn sig_utils() {
        let mut rng = StdRng::seed_from_u64(8349u64);
        let secret = SecretKey::<RedJubjub>::new(Fp::random(&mut rng));
        let td = TransactionData::new(
            secret.to_public_key(),
            PublicKey::new(JubjubExtended::random(&mut rng)),
            10,
        );

        let user = UserData::new(0, 10, secret.to_public_key());

        let t = td.signed(secret, &mut rng);

        let td_bytes = td.to_bytes();
        let td_back = TransactionData::from_bytes(td_bytes).unwrap();
        assert_eq!(td, td_back);

        let t_bytes = t.to_bytes();
        let t_back = Transaction::from_bytes(t_bytes).unwrap();
        assert_eq!(t, t_back);

        let user_bytes = user.to_bytes();
        let user_back = UserData::from_bytes(user_bytes).unwrap();
        assert_eq!(user, user_back);
    }
}
