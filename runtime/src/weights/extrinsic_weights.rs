//! # FintradeX Extrinsic Weights
//!
//! This module defines the weight calculations for individual transactions (extrinsics)
//! on the FintradeX parachain, providing cost estimates for all operations.
//!
//! ## Weight Categories
//!
//! - **Trading Operations**: Order placement, matching, and execution
//! - **Asset Management**: Cross-chain transfers and conversions
//! - **Governance**: Voting and proposal operations
//! - **EVM Operations**: Smart contract execution costs
//!
//! ## Cost Optimization
//!
//! - Competitive transaction fees
//! - Efficient resource utilization
//! - Optimized for DeFi protocols
//! - Scalable transaction processing
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
        /// Executing a NO-OP `System::remarks` Extrinsic.
        pub const ExtrinsicBaseWeight: Weight =
            Weight::from_parts(constants::WEIGHT_REF_TIME_PER_NANOS.saturating_mul(125_000), 0);
    }

    #[cfg(test)]
    mod test_weights {
        use polkadot_sdk::*;

        use frame_support::weights::constants;

        /// Checks that the weight exists and is sane.
        // NOTE: If this test fails but you are sure that the generated values are fine,
        // you can delete it.
        #[test]
        fn sane() {
            let w = super::constants::ExtrinsicBaseWeight::get();

            // At least 10 µs.
            assert!(
                w.ref_time() >= 10u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
                "Weight should be at least 10 µs."
            );
            // At most 1 ms.
            assert!(
                w.ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
                "Weight should be at most 1 ms."
            );
        }
    }
}
