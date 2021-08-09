// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_eth_sub_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-06, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_eth_sub_bridge
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/bridge/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_eth_sub_bridge.
pub trait WeightInfo {
    fn set_transfer() -> Weight;
    fn multi_signed_mint() -> Weight;
    fn update_limits() -> Weight;
    fn approve_transfer() -> Weight;
    fn update_validator_list() -> Weight;
    fn pause_bridge() -> Weight;
    fn resume_bridge() -> Weight;
    fn confirm_transfer() -> Weight;
    fn cancel_transfer() -> Weight;
}

/// Weights for pallet_eth_sub_bridge using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_transfer() -> Weight {
        (105_715_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn multi_signed_mint() -> Weight {
        (134_816_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(12 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn update_limits() -> Weight {
        (84_833_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn approve_transfer() -> Weight {
        (150_397_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(13 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn update_validator_list() -> Weight {
        (98_845_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(9 as Weight))
    }
    fn pause_bridge() -> Weight {
        (86_714_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn resume_bridge() -> Weight {
        (87_924_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn confirm_transfer() -> Weight {
        (162_017_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(12 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn cancel_transfer() -> Weight {
        (146_398_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn set_transfer() -> Weight {
        (105_715_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(8 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn multi_signed_mint() -> Weight {
        (134_816_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(12 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
    }
    fn update_limits() -> Weight {
        (84_833_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn approve_transfer() -> Weight {
        (150_397_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(13 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn update_validator_list() -> Weight {
        (98_845_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(9 as Weight))
    }
    fn pause_bridge() -> Weight {
        (86_714_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn resume_bridge() -> Weight {
        (87_924_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn confirm_transfer() -> Weight {
        (162_017_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(12 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn cancel_transfer() -> Weight {
        (146_398_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
}
