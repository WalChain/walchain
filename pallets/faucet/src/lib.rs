#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{
    Currency, ExistenceRequirement::KeepAlive, Get, Imbalance, OnUnbalanced, ReservableCurrency,
    WithdrawReasons,
};
use frame_support::weights::Weight;
use frame_support::{print, PalletId};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    #[pallet::config]
    pub trait Config: pallet_timestamp::Config + frame_system::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type DripAmount: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MinBlocksBetweenClaims: Get<<Self as pallet_timestamp::Config>::Moment>;

        #[pallet::constant]
        type MaxClaimsPerAccount: Get<u32>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // #[pallet::storage]
    // #[pallet::getter(fn something)]
    // // Learn more about declaring storage items:
    // // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    // pub type Something<T> = StorageValue<_, u32>;

    // #[pallet::storage]
    // #[pallet::getter(fn lastClaimsOf)]
    // pub(super) type LastClaimsOf<T> =
    //     StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // / Event documentation should end with an array that provides descriptive names for event
        // / parameters. [something, who]
        // SomethingStored(u32, T::AccountId),
        FaucetDripped(BalanceOf<T>, T::AccountId),
    }

    // // Errors inform users that something went wrong.
    // #[pallet::error]
    // pub enum Error<T> {
    // 	/// Error names should be descriptive.
    // 	NoneValue,
    // 	/// Errors should have helpful documentation associated with them.
    // 	StorageOverflow,
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        // pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
        // 	// Check that the extrinsic was signed and get the signer.
        // 	// This function will return an error if the extrinsic is not signed.
        // 	// https://substrate.dev/docs/en/knowledgebase/runtime/origin
        // 	let who = ensure_signed(origin)?;

        // 	// Update storage.
        // 	<Something<T>>::put(something);

        // 	// Emit an event.
        // 	Self::deposit_event(Event::SomethingStored(something, who));
        // 	// Return a successful DispatchResultWithPostInfo
        // 	Ok(())
        // }

        #[pallet::weight(10_000 /*+ T::DbWeight::get().writes(1)*/)]
        pub fn claim_tokens(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify account's last claim was not less MinBlockBetweenClaims ago
            // Verify account's total claims does not exceed MaxClaimsPerAccount

            let imbalance = T::Currency::deposit_creating(&who, T::DripAmount::get());
            drop(imbalance);

            Self::deposit_event(Event::FaucetDripped(T::DripAmount::get(), who));

            Ok(())
        }

        // /// An example dispatchable that may throw a custom error.
        // #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        // pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
        // 	let _who = ensure_signed(origin)?;

        // 	// Read a value from storage.
        // 	match <Something<T>>::get() {
        // 		// Return an error if the value has not been set.
        // 		None => Err(Error::<T>::NoneValue)?,
        // 		Some(old) => {
        // 			// Increment the value read from storage; will error in the event of overflow.
        // 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
        // 			// Update the value in storage with the incremented result.
        // 			<Something<T>>::put(new);
        // 			Ok(())
        // 		},
        // 	}
        // }
    }
}
