// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Autogenerated weights for `pallet_price_feed`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pint-local"), DB CACHE: 128

// Executed Command:
// ./target/release/pint
// benchmark
// -p
// pallet_price_feed
// -e
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --raw
// --chain
// pint-local
// --output
// ./runtime/common/src/weights/pallet_price_feed.rs
// --steps
// 50
// --repeat
// 20
// --heap-pages
// 4096


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_price_feed.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_price_feed::WeightInfo for WeightInfo<T> {
	// Storage: PriceFeed AssetFeeds (r:1 w:1)
	fn map_asset_price_feed() -> Weight {
		(15_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PriceFeed AssetFeeds (r:1 w:1)
	fn unmap_asset_price_feed() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}