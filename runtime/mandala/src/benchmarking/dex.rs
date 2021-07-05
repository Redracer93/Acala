// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
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

use crate::{dollar, AccountId, Balance, Currencies, CurrencyId, Dex, Runtime, TradingPathLimit};

use frame_benchmarking::{account, whitelisted_caller};
use frame_system::RawOrigin;
use orml_benchmarking::runtime_benchmarks;
use orml_traits::MultiCurrencyExtended;
use primitives::{
	currency::{AUSD, RENBTC},
	TradingPair,
};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::prelude::*;

const SEED: u32 = 0;

fn inject_liquidity(
	maker: AccountId,
	currency_id_a: CurrencyId,
	currency_id_b: CurrencyId,
	max_amount_a: Balance,
	max_amount_b: Balance,
	deposit: bool,
) -> Result<(), &'static str> {
	// set balance
	<Currencies as MultiCurrencyExtended<_>>::update_balance(
		currency_id_a,
		&maker,
		max_amount_a.unique_saturated_into(),
	)?;
	<Currencies as MultiCurrencyExtended<_>>::update_balance(
		currency_id_b,
		&maker,
		max_amount_b.unique_saturated_into(),
	)?;

	let _ = Dex::enable_trading_pair(RawOrigin::Root.into(), currency_id_a, currency_id_b);

	Dex::add_liquidity(
		RawOrigin::Signed(maker.clone()).into(),
		currency_id_a,
		currency_id_b,
		max_amount_a,
		max_amount_b,
		Default::default(),
		deposit,
	)?;

	Ok(())
}

runtime_benchmarks! {
	{ Runtime, module_dex }

	// enable a Disabled trading pair
	enable_trading_pair {
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
	}: _(RawOrigin::Root, trading_pair.0, trading_pair.1)

	// disable a Enabled trading pair
	disable_trading_pair {
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::enable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
	}: _(RawOrigin::Root, trading_pair.0, trading_pair.1)

	// list a Provisioning trading pair
	list_provisioning {
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
	}: _(RawOrigin::Root, trading_pair.0, trading_pair.1, dollar(trading_pair.0), dollar(trading_pair.1), dollar(trading_pair.0), dollar(trading_pair.1), 10)

	// update parameters of a Provisioning trading pair
	update_provisioning_parameters {
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
		Dex::list_provisioning(
			RawOrigin::Root.into(),
			trading_pair.0,
			trading_pair.1,
			dollar(trading_pair.0),
			dollar(trading_pair.1),
			100 * dollar(trading_pair.0),
			1000 * dollar(trading_pair.1),
			100
		)?;
	}: _(RawOrigin::Root, trading_pair.0, trading_pair.1, 2 * dollar(trading_pair.0), 2 * dollar(trading_pair.1), 10 * dollar(trading_pair.0), 100 * dollar(trading_pair.1), 200)

	// end a Provisioning trading pair
	end_provisioning {
		let founder: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
		Dex::list_provisioning(
			RawOrigin::Root.into(),
			trading_pair.0,
			trading_pair.1,
			dollar(trading_pair.0),
			dollar(trading_pair.1),
			100 * dollar(trading_pair.0),
			100 * dollar(trading_pair.1),
			0
		)?;

		// set balance
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.0, &founder, (100 * dollar(trading_pair.0)).unique_saturated_into())?;
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.1, &founder, (100 * dollar(trading_pair.1)).unique_saturated_into())?;

		// add enough provision
		Dex::add_provision(
			RawOrigin::Signed(founder.clone()).into(),
			trading_pair.0,
			trading_pair.1,
			100 * dollar(trading_pair.0),
			100 * dollar(trading_pair.1),
		)?;
	}: _(RawOrigin::Signed(founder), trading_pair.0, trading_pair.1)

	add_provision {
		let founder: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
		Dex::list_provisioning(
			RawOrigin::Root.into(),
			trading_pair.0,
			trading_pair.1,
			dollar(trading_pair.0),
			dollar(trading_pair.1),
			100 * dollar(trading_pair.0),
			1000 * dollar(trading_pair.1),
			0
		)?;

		// set balance
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.0, &founder, (10 * dollar(trading_pair.0)).unique_saturated_into())?;
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.1, &founder, (10 * dollar(trading_pair.1)).unique_saturated_into())?;
	}: _(RawOrigin::Signed(founder), trading_pair.0, trading_pair.1, dollar(trading_pair.0), dollar(trading_pair.1))

	claim_dex_share {
		let founder: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let _ = Dex::disable_trading_pair(RawOrigin::Root.into(), trading_pair.0, trading_pair.1);
		Dex::list_provisioning(
			RawOrigin::Root.into(),
			trading_pair.0,
			trading_pair.1,
			dollar(trading_pair.0),
			dollar(trading_pair.1),
			10 * dollar(trading_pair.0),
			10 * dollar(trading_pair.1),
			0
		)?;

		// set balance
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.0, &founder, (100 * dollar(trading_pair.0)).unique_saturated_into())?;
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.1, &founder, (100 * dollar(trading_pair.1)).unique_saturated_into())?;

		Dex::add_provision(
			RawOrigin::Signed(founder.clone()).into(),
			trading_pair.0,
			trading_pair.1,
			dollar(trading_pair.0),
			20 * dollar(trading_pair.1)
		)?;
		Dex::end_provisioning(
			RawOrigin::Signed(founder.clone()).into(),
			trading_pair.0,
			trading_pair.1,
		)?;
	}: _(RawOrigin::Signed(founder), trading_pair.0, trading_pair.1)

	// add liquidity but don't staking lp
	add_liquidity {
		let first_maker: AccountId = account("first_maker", 0, SEED);
		let second_maker: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let amount_a = 100 * dollar(trading_pair.0);
		let amount_b = 10_000 * dollar(trading_pair.1);

		// set balance
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.0, &second_maker, amount_a.unique_saturated_into())?;
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.1, &second_maker, amount_b.unique_saturated_into())?;

		// first maker inject liquidity
		inject_liquidity(first_maker.clone(), trading_pair.0, trading_pair.1, amount_a, amount_b, false)?;
	}: add_liquidity(RawOrigin::Signed(second_maker), trading_pair.0, trading_pair.1, amount_a, amount_b, Default::default(), false)

	// worst: add liquidity and stake lp
	add_liquidity_and_stake {
		let first_maker: AccountId = account("first_maker", 0, SEED);
		let second_maker: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let amount_a = 100 * dollar(trading_pair.0);
		let amount_b = 10_000 * dollar(trading_pair.1);

		// set balance
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.0, &second_maker, amount_a.unique_saturated_into())?;
		<Currencies as MultiCurrencyExtended<_>>::update_balance(trading_pair.1, &second_maker, amount_b.unique_saturated_into())?;

		// first maker inject liquidity
		inject_liquidity(first_maker.clone(), trading_pair.0, trading_pair.1, amount_a, amount_b, true)?;
	}: add_liquidity(RawOrigin::Signed(second_maker), trading_pair.0, trading_pair.1, amount_a, amount_b, Default::default(), true)

	// remove liquidity by liquid lp share
	remove_liquidity {
		let maker: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		inject_liquidity(maker.clone(), trading_pair.0, trading_pair.1, 100 * dollar(trading_pair.0), 10_000 * dollar(trading_pair.1), false)?;
	}: remove_liquidity(RawOrigin::Signed(maker), trading_pair.0, trading_pair.1, 50 * dollar(trading_pair.0), Default::default(), Default::default(), false)

	// remove liquidity by withdraw staking lp share
	remove_liquidity_by_unstake {
		let maker: AccountId = whitelisted_caller();
		let trading_pair = TradingPair::new(AUSD, RENBTC);
		inject_liquidity(maker.clone(), trading_pair.0, trading_pair.1, 100 * dollar(trading_pair.0), 10_000 * dollar(trading_pair.1), true)?;
	}: remove_liquidity(RawOrigin::Signed(maker), trading_pair.0, trading_pair.1, 50 * dollar(trading_pair.0), Default::default(), Default::default(), true)

	swap_with_exact_supply {
		let u in 2 .. TradingPathLimit::get() as u32;

		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let mut path: Vec<CurrencyId> = vec![];
		for i in 1 .. u {
			if i == 1 {
				path.push(trading_pair.0);
				path.push(trading_pair.1);
			} else {
				if i % 2 == 0 {
					path.push(trading_pair.0);
				} else {
					path.push(trading_pair.1);
				}
			}
		}

		let maker: AccountId = account("maker", 0, SEED);
		let taker: AccountId = whitelisted_caller();
		inject_liquidity(maker, trading_pair.0, trading_pair.1, 10_000 * dollar(trading_pair.0), 10_000 * dollar(trading_pair.1), false)?;

		<Currencies as MultiCurrencyExtended<_>>::update_balance(path[0], &taker, (10_000 * dollar(path[0])).unique_saturated_into())?;
	}: swap_with_exact_supply(RawOrigin::Signed(taker), path.clone(), 100 * dollar(path[0]), 0)

	swap_with_exact_target {
		let u in 2 .. TradingPathLimit::get() as u32;

		let trading_pair = TradingPair::new(AUSD, RENBTC);
		let mut path: Vec<CurrencyId> = vec![];
		for i in 1 .. u {
			if i == 1 {
				path.push(trading_pair.0);
				path.push(trading_pair.1);
			} else {
				if i % 2 == 0 {
					path.push(trading_pair.0);
				} else {
					path.push(trading_pair.1);
				}
			}
		}

		let maker: AccountId = account("maker", 0, SEED);
		let taker: AccountId = whitelisted_caller();
		inject_liquidity(maker, trading_pair.0, trading_pair.1, 10_000 * dollar(trading_pair.0), 10_000 * dollar(trading_pair.1), false)?;

		<Currencies as MultiCurrencyExtended<_>>::update_balance(path[0], &taker, (10_000 * dollar(path[0])).unique_saturated_into())?;
	}: swap_with_exact_target(RawOrigin::Signed(taker), path.clone(), 10 * dollar(path[path.len() - 1]), 100 * dollar(path[0]))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use orml_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(new_test_ext(),);
}
