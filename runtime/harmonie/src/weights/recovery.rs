
//! Autogenerated weights for pallet_recovery
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `symphonie-node-3`, CPU: `Intel Xeon Processor (Skylake, IBRS)`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_recovery
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/recovery.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_recovery.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for pallet_recovery using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_recovery::WeightInfo for AllfeatWeight<T> {
	/// Storage: `Recovery::Proxy` (r:1 w:0)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 46_641_000 picoseconds.
		Weight::from_parts(49_262_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Recovery::Proxy` (r:0 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 20_887_000 picoseconds.
		Weight::from_parts(21_707_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3816`
		// Minimum execution time: 59_263_000 picoseconds.
		Weight::from_parts(149_364_264, 3816)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3854`
		// Minimum execution time: 69_575_000 picoseconds.
		Weight::from_parts(167_204_000, 3854)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 41_267_000 picoseconds.
		Weight::from_parts(91_669_667, 3854)
			// Standard Error: 261_227
			.saturating_add(Weight::from_parts(1_361_268, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Recoverable` (r:1 w:0)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + n * (64 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 51_534_000 picoseconds.
		Weight::from_parts(113_614_783, 3854)
			// Standard Error: 284_561
			.saturating_add(Weight::from_parts(425_730, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:1)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 75_313_000 picoseconds.
		Weight::from_parts(190_256_047, 3854)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Recovery::ActiveRecoveries` (r:1 w:0)
	/// Proof: `Recovery::ActiveRecoveries` (`max_values`: None, `max_size`: Some(389), added: 2864, mode: `MaxEncodedLen`)
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(351), added: 2826, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + n * (32 ±0)`
		//  Estimated: `3854`
		// Minimum execution time: 70_413_000 picoseconds.
		Weight::from_parts(144_066_401, 3854)
			// Standard Error: 571_343
			.saturating_add(Weight::from_parts(1_769_625, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Recovery::Proxy` (r:1 w:1)
	/// Proof: `Recovery::Proxy` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148`
		//  Estimated: `3545`
		// Minimum execution time: 45_584_000 picoseconds.
		Weight::from_parts(67_654_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}