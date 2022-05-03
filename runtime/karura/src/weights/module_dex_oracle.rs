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

//! Autogenerated weights for module_dex_oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
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

/// Weight functions for module_dex_oracle.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex_oracle::WeightInfo for WeightInfo<T> {
	// Storage: DexOracle AveragePrices (r:1 w:0)
	// Storage: Timestamp Now (r:0 w:1)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: DexOracle Cumulatives (r:1 w:1)
	fn on_initialize_with_update_average_prices(n: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 218_000
			.saturating_add((17_412_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 218_000
			.saturating_add((11_556_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(u as Weight)))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: DexOracle Cumulatives (r:0 w:1)
	fn enable_average_price() -> Weight {
		(11_430_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	// Storage: DexOracle Cumulatives (r:0 w:1)
	fn disable_average_price() -> Weight {
		(6_452_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	fn update_average_price_interval() -> Weight {
		(6_362_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
