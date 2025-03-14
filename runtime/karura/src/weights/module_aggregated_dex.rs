// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_aggregated_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `187e78510d7a`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_aggregated_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_aggregated_dex::WeightInfo for WeightInfo<T> {
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Minimum execution time: 72_360 nanoseconds.
		Weight::from_ref_time(53_352_737)
			// Standard Error: 66_097
			.saturating_add(Weight::from_ref_time(11_170_297).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_target(u: u32, ) -> Weight {
		// Minimum execution time: 81_156 nanoseconds.
		Weight::from_ref_time(50_997_808)
			// Standard Error: 78_781
			.saturating_add(Weight::from_ref_time(16_696_343).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
	}
	// Storage: AggregatedDex AggregatedSwapPaths (r:0 w:1)
	/// The range of component `n` is `[0, 7]`.
	fn update_aggregated_swap_paths(n: u32, ) -> Weight {
		// Minimum execution time: 4_508 nanoseconds.
		Weight::from_ref_time(4_091_749)
			// Standard Error: 9_592
			.saturating_add(Weight::from_ref_time(1_518_229).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
}
