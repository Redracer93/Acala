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

//! Cross-chain transfer tests within Polkadot network.

use crate::relaychain::fee_test::*;
use crate::relaychain::polkadot_test_net::*;
use crate::setup::*;

use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_core::bounded::BoundedVec;
use xcm::VersionedXcm;
use xcm_emulator::TestExt;

pub const ACALA_ID: u32 = 2000;
pub const MOCK_BIFROST_ID: u32 = 2001;

fn bifrost_reserve_account() -> AccountId {
	polkadot_parachain::primitives::Sibling::from(MOCK_BIFROST_ID).into_account_truncating()
}

#[test]
fn token_per_second_works() {
	let aca_per_second = acala_runtime::aca_per_second();
	assert_eq!(10_016_000_000_000, aca_per_second);

	let dot_per_second = acala_runtime::dot_per_second();
	assert_eq!(2_003_200_000, dot_per_second);
}

#[test]
fn transfer_from_relay_chain() {
	PolkadotNet::execute_with(|| {
		assert_ok!(polkadot_runtime::XcmPallet::reserve_transfer_assets(
			polkadot_runtime::RuntimeOrigin::signed(ALICE.into()),
			Box::new(Parachain(ACALA_ID).into_versioned()),
			Box::new(Junction::AccountId32 { id: BOB, network: None }.into_versioned()),
			Box::new((Here, dollar(DOT)).into()),
			0
		));
	});

	Acala::execute_with(|| {
		assert_eq!(9_998_397_440, Tokens::free_balance(DOT, &AccountId::from(BOB)));
	});
}

#[test]
fn transfer_to_relay_chain() {
	Acala::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			DOT,
			5 * dollar(DOT),
			Box::new(MultiLocation::new(1, X1(Junction::AccountId32 { id: BOB, network: None })).into()),
			WeightLimit::Unlimited
		));
	});
	PolkadotNet::execute_with(|| {
		assert_eq!(
			// v0.9.19: 49_517_228_896
			// v0.9.22: 49_530_582_548
			// v0.9.31: 49_573_469_824
			// v0.9.33: 49_581_059_712
			// v0.9.36: 49_591_353_032
			// v0.9.37: 49_578_565_860
			// v0.9.38: 49_637_471_000
			49_637_471_000,
			polkadot_runtime::Balances::free_balance(&AccountId::from(BOB))
		);
		assert_eq!(
			5 * dollar(DOT),
			polkadot_runtime::Balances::free_balance(&ParaId::from(ACALA_ID).into_account_truncating())
		);
	});
}

#[test]
fn liquid_crowdloan_xtokens_works() {
	TestNet::reset();
	let foreign_asset = CurrencyId::ForeignAsset(0);
	let dollar = dollar(KAR);
	let minimal_balance = Balances::minimum_balance() / 10; // 10%
	let foreign_fee = foreign_per_second_as_fee(4, minimal_balance);

	MockBifrost::execute_with(|| {
		assert_ok!(AssetRegistry::register_foreign_asset(
			RuntimeOrigin::root(),
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(ACALA_ID),
						Junction::from(BoundedVec::try_from(LCDOT.encode()).unwrap())
					)
				)
				.into()
			),
			Box::new(AssetMetadata {
				name: b"Liquid Crowdloan Token".to_vec(),
				symbol: b"LCDOT".to_vec(),
				decimals: 12,
				minimal_balance
			})
		));
	});

	Acala::execute_with(|| {
		assert_ok!(AssetRegistry::register_native_asset(
			RuntimeOrigin::root(),
			LCDOT,
			Box::new(AssetMetadata {
				name: b"Liquid Crowdloan Token".to_vec(),
				symbol: b"LCDOT".to_vec(),
				decimals: 12,
				minimal_balance
			})
		));
		assert_ok!(Tokens::deposit(LCDOT, &AccountId::from(BOB), 10 * dollar));

		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(BOB.into()),
			LCDOT,
			5 * dollar,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(MOCK_BIFROST_ID),
						Junction::AccountId32 {
							network: None,
							id: ALICE.into(),
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(XcmWeight::from_ref_time(8_000_000_000)),
		));

		assert_eq!(Tokens::free_balance(LCDOT, &AccountId::from(BOB)), 5 * dollar);
		assert_eq!(Tokens::free_balance(LCDOT, &bifrost_reserve_account()), 5 * dollar);
	});

	MockBifrost::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(foreign_asset, &AccountId::from(ALICE)),
			5 * dollar - foreign_fee
		);

		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			foreign_asset,
			dollar,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(ACALA_ID),
						Junction::AccountId32 {
							network: None,
							id: BOB.into(),
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(XcmWeight::from_ref_time(8_000_000_000)),
		));
	});

	Acala::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(LCDOT, &AccountId::from(BOB)),
			6 * dollar - foreign_fee
		);
		assert_eq!(Tokens::free_balance(LCDOT, &bifrost_reserve_account()), 4 * dollar);
	});
}

#[test]
fn send_arbitrary_xcm_fails() {
	TestNet::reset();

	Acala::execute_with(|| {
		assert_noop!(
			PolkadotXcm::send(
				acala_runtime::RuntimeOrigin::signed(ALICE.into()),
				Box::new(MultiLocation::new(1, Here).into()),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, 1).into())]))),
			),
			BadOrigin
		);
	});
}
