#![cfg(test)]

#[macro_export]
macro_rules! decl_tests {
    ($test:ty, $ext_builder:ty) => {
        use frame_support::{assert_noop, assert_ok};
        use frame_system::RawOrigin;
        use jub_jub::Fp;
        use sp_runtime::traits::BadOrigin;
        use $crate::*;

        const ID_1_PK: Fp = Fp::to_mont_form([1, 0, 0, 0]);
        const ID_2_PK: Fp = Fp::to_mont_form([2, 0, 0, 0]);
        const ID_1_RANDOM: Fp = Fp::to_mont_form([1, 2, 3, 4]);
        const ID_2_RANDOM: Fp = Fp::to_mont_form([4, 3, 2, 1]);
        const TRANSFER_RANDOM: Fp = Fp::to_mont_form([4, 2, 1, 3]);

        #[test]
        fn balance_transfer_works() {
            let balance1 = 50;
            let transfer = 20;
            let enc_balance1 = EncryptedNumber::encrypt(ID_1_PK, balance1, ID_1_RANDOM);
            let enc_balance2 = EncryptedNumber::encrypt(ID_2_PK, 0, ID_2_RANDOM);
            let enc_transfer = EncryptedNumber::encrypt(ID_1_PK, transfer, TRANSFER_RANDOM);

            <$ext_builder>::default().build().execute_with(|| {
                let _ = EncryptedBalances::deposit_creating(&1, enc_balance1);
                assert_ok!(EncryptedBalances::transfer(
                    Some(1).into(),
                    2,
                    enc_transfer,
                    enc_transfer
                ));
                assert_eq!(
                    EncryptedBalances::total_balance(&1),
                    enc_balance1 - enc_transfer
                );
                assert_eq!(
                    EncryptedBalances::total_balance(&2),
                    enc_balance2 + enc_transfer
                );
            });
        }

        #[test]
        fn force_transfer_works() {
            let balance1 = 50;
            let transfer = 20;
            let enc_balance1 = EncryptedNumber::encrypt(ID_1_PK, balance1, ID_1_RANDOM);
            let enc_balance2 = EncryptedNumber::encrypt(ID_2_PK, 0, ID_2_RANDOM);
            let enc_transfer = EncryptedNumber::encrypt(ID_1_PK, transfer, TRANSFER_RANDOM);
            <$ext_builder>::default().build().execute_with(|| {
                let _ = EncryptedBalances::deposit_creating(&1, enc_balance1);
                assert_noop!(
                    EncryptedBalances::force_transfer(
                        Some(2).into(),
                        1,
                        2,
                        enc_transfer,
                        enc_transfer
                    ),
                    BadOrigin,
                );
                assert_ok!(EncryptedBalances::force_transfer(
                    RawOrigin::Root.into(),
                    1,
                    2,
                    enc_transfer,
                    enc_transfer
                ));
                assert_eq!(
                    EncryptedBalances::total_balance(&1),
                    enc_balance1 - enc_transfer
                );
                assert_eq!(
                    EncryptedBalances::total_balance(&2),
                    enc_balance2 + enc_transfer
                );
            });
        }
    };
}
