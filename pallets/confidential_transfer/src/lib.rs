#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_plonk::{FullcodecRng, Proof};
    use sp_runtime::traits::StaticLookup;
    use zero_circuits::ConfidentialTransferTransaction;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_plonk::Config + pallet_encrypted_balance::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            degree: u32,
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            pallet_plonk::Pallet::<T>::trusted_setup(origin, degree, rng)?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn confidential_transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            proof: Proof,
            transaction_params: ConfidentialTransferTransaction<T::EncryptedBalance>,
        ) -> DispatchResultWithPostInfo {
            let public_inputs = transaction_params.clone().public_inputs();
            pallet_plonk::Pallet::<T>::verify(origin.clone(), proof, public_inputs.to_vec())?;
            pallet_encrypted_balance::Pallet::<T>::transfer(
                origin,
                dest,
                transaction_params.sender_encrypted_transfer_amount,
            )?;
            Ok(().into())
        }
    }
}
