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

//! Autogenerated weights for module_honzon_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

/// Weight functions for module_honzon_bridge.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon_bridge::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	fn to_bridged() -> Weight {
		(10_261_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	fn from_bridged() -> Weight {
		(10_341_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
