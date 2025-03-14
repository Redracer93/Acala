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

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `295f33c1d5e7`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn enable_trading_pair() -> Weight {
		// Minimum execution time: 16_890 nanoseconds.
		Weight::from_ref_time(17_331_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn disable_trading_pair() -> Weight {
		// Minimum execution time: 17_988 nanoseconds.
		Weight::from_ref_time(18_452_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:0)
	fn list_provisioning() -> Weight {
		// Minimum execution time: 23_197 nanoseconds.
		Weight::from_ref_time(23_858_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn update_provisioning_parameters() -> Weight {
		// Minimum execution time: 10_315 nanoseconds.
		Weight::from_ref_time(10_948_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Dex InitialShareExchangeRates (r:0 w:1)
	fn end_provisioning() -> Weight {
		// Minimum execution time: 46_167 nanoseconds.
		Weight::from_ref_time(48_094_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn add_provision() -> Weight {
		// Minimum execution time: 74_900 nanoseconds.
		Weight::from_ref_time(76_656_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:2 w:1)
	// Storage: Dex InitialShareExchangeRates (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn claim_dex_share() -> Weight {
		// Minimum execution time: 66_817 nanoseconds.
		Weight::from_ref_time(69_164_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 91_092 nanoseconds.
		Weight::from_ref_time(94_294_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	fn add_liquidity_and_stake() -> Weight {
		// Minimum execution time: 128_056 nanoseconds.
		Weight::from_ref_time(131_607_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 85_988 nanoseconds.
		Weight::from_ref_time(88_510_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn remove_liquidity_by_unstake() -> Weight {
		// Minimum execution time: 139_350 nanoseconds.
		Weight::from_ref_time(141_724_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Minimum execution time: 71_241 nanoseconds.
		Weight::from_ref_time(52_471_993)
			// Standard Error: 61_943
			.saturating_add(Weight::from_ref_time(10_676_220).saturating_mul(u.into()))
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
		// Minimum execution time: 71_427 nanoseconds.
		Weight::from_ref_time(52_512_131)
			// Standard Error: 62_644
			.saturating_add(Weight::from_ref_time(10_710_101).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex InitialShareExchangeRates (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn refund_provision() -> Weight {
		// Minimum execution time: 76_682 nanoseconds.
		Weight::from_ref_time(78_334_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn abort_provisioning() -> Weight {
		// Minimum execution time: 21_801 nanoseconds.
		Weight::from_ref_time(22_759_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
