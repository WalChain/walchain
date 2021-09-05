//! Benchmarking setup for pallet-faucet

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Pallet as Faucet;

benchmarks! {
	claim_tokens {
		let s in 0 .. 100;
		let caller: T::AccountId = account("caller", s, 0);
	}: _(RawOrigin::Signed(caller))
	// verify {
	// 	assert_eq!(LastClaimOf::<T>::get(caller), Some((1,1.into())));
	// }
}

impl_benchmark_test_suite!(
	Faucet,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
