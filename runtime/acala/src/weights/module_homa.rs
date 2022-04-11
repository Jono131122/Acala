// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

//! Autogenerated weights for module_homa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
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

/// Weight functions for module_homa.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_homa::WeightInfo for WeightInfo<T> {
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Homa LastEraBumpedBlock (r:1 w:0)
	// Storage: Homa BumpEraFrequency (r:1 w:0)
	fn on_initialize() -> Weight {
		(4_249_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Homa LastEraBumpedBlock (r:1 w:0)
	// Storage: Homa BumpEraFrequency (r:1 w:0)
	fn on_initialize_with_bump_era() -> Weight {
		(6_765_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: Homa StakingLedgers (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:1)
	// Storage: Homa SoftBondedCapPerSubAccount (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Homa TotalVoidLiquid (r:1 w:1)
	// Storage: Homa EstimatedRewardRatePerEra (r:1 w:0)
	fn mint() -> Weight {
		(68_461_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Homa RedeemRequests (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn request_redeem() -> Weight {
		(36_972_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Homa RedeemRequests (r:1 w:1)
	// Storage: Homa ToBondPool (r:1 w:1)
	// Storage: Homa StakingLedgers (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: Homa FastMatchFeeRate (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:2 w:2)
	fn fast_match_redeems(n: u32, ) -> Weight {
		(22_847_000 as Weight)
			// Standard Error: 77_000
			.saturating_add((48_950_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Homa RelayChainCurrentEra (r:1 w:0)
	// Storage: Homa Unbondings (r:2 w:1)
	// Storage: Homa UnclaimedRedemption (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts Accounts (r:0 w:1)
	fn claim_redemption() -> Weight {
		(62_190_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Homa SoftBondedCapPerSubAccount (r:0 w:1)
	// Storage: Homa FastMatchFeeRate (r:0 w:1)
	// Storage: Homa CommissionRate (r:0 w:1)
	// Storage: Homa EstimatedRewardRatePerEra (r:0 w:1)
	fn update_homa_params() -> Weight {
		(31_300_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Homa LastEraBumpedBlock (r:0 w:1)
	// Storage: Homa BumpEraFrequency (r:0 w:1)
	fn update_bump_era_params() -> Weight {
		(14_042_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Homa StakingLedgers (r:1 w:1)
	fn reset_ledgers(n: u32, ) -> Weight {
		(5_448_000 as Weight)
			// Standard Error: 44_000
			.saturating_add((7_667_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Homa RelayChainCurrentEra (r:1 w:1)
	fn reset_current_era() -> Weight {
		(10_690_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
