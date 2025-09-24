//! # FintradeX Block Weights
//!
//! This module defines the block weight limits and execution time constraints
//! for the FintradeX parachain, optimized for high-performance trading operations.
//!
//! ## Block Configuration
//!
//! - **Max Block Weight**: Optimized for trading transactions
//! - **Execution Time**: 3-second block time for fast finality
//! - **Transaction Limits**: High-capacity transaction processing
//! - **Resource Allocation**: Balanced for trading and DeFi operations
//!
//! ## Performance Features
//!
//! - Sub-second transaction processing
//! - High-throughput order matching
//! - Efficient cross-chain operations
//! - Competitive gas costs
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

pub mod constants {
	use polkadot_sdk::*;

	use frame_support::{
		parameter_types,
		weights::{constants, Weight},
	};

	parameter_types! {
		/// Importing a block with 0 Extrinsics.
		pub const BlockExecutionWeight: Weight =
			Weight::from_parts(constants::WEIGHT_REF_TIME_PER_NANOS.saturating_mul(5_000_000), 0);
		
			
	}
	pub const WEIGHT_MILLISECS_PER_BLOCK: u64 = 2000;
	#[cfg(test)]
	mod test_weights {
		use polkadot_sdk::*;

		use frame_support::weights::constants;

		/// Checks that the weight exists and is sane.
		// NOTE: If this test fails but you are sure that the generated values are fine,
		// you can delete it.
		#[test]
		fn sane() {
			let w = super::constants::BlockExecutionWeight::get();

			// At least 100 µs.
			assert!(
				w.ref_time() >= 100u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
				"Weight should be at least 100 µs."
			);
			// At most 50 ms.
			assert!(
				w.ref_time() <= 50u64 * constants::WEIGHT_REF_TIME_PER_MILLIS,
				"Weight should be at most 50 ms."
			);
		}
	}
}
