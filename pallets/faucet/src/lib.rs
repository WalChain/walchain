#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, Get, ReservableCurrency};
use frame_support::transactional;
use sp_runtime::traits::Saturating;

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
    use sp_runtime::ArithmeticError;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type DripAmount: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MinBlocksBetweenClaims: Get<<Self as frame_system::Config>::BlockNumber>;

        #[pallet::constant]
        type MaxClaimsPerAccount: Get<u32>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    

    #[pallet::storage]
    #[pallet::getter(fn last_claim_of)]
    pub(super) type LastClaimOf<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Option<(u32, T::BlockNumber)>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Faucet has dripped tokens to account [balance, who]
        FaucetDripped(BalanceOf<T>, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        LastClaimTooRecent,
        MaxClaimsExceeded,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn claim_tokens(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let current_block = <frame_system::Pallet<T>>::block_number();

            <LastClaimOf<T>>::try_mutate(&who, |last_claim| -> DispatchResult {
                let current_claim = match last_claim {
                    Some((claim_count, last_claim_block)) => {
                        let new_claim_count = claim_count
                            .checked_add(1)
                            .ok_or(ArithmeticError::Overflow)?;
                        // Verify account's total claims does not exceed MaxClaimsPerAccount
                        ensure!(
                            new_claim_count <= T::MaxClaimsPerAccount::get(),
                            Error::<T>::MaxClaimsExceeded
                        );
                        // Verify account's last claim was not less MinBlockBetweenClaims ago
                        ensure!(
                            last_claim_block.saturating_add(T::MinBlocksBetweenClaims::get())
                                < current_block,
                            Error::<T>::LastClaimTooRecent
                        );
                        (new_claim_count, current_block)
                    }
                    None => (1, current_block),
                };
                *last_claim = Some(current_claim);
                Ok(())
            })?;

            let imbalance = T::Currency::deposit_creating(&who, T::DripAmount::get());
            drop(imbalance);

            Self::deposit_event(Event::FaucetDripped(T::DripAmount::get(), who));

            Ok(())
        }
    }
}
