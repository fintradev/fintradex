//! # FintradeX Runtime Constants
//!
//! This module defines the core constants used throughout the FintradeX runtime,
//! including currency denominations, time parameters, and system configurations.
//!
//! ## Currency Constants
//!
//! The FintradeX token uses a precision of 15 decimal places, with the following denominations:
//! - `MILLI_FINTS`: 1,000,000,000 (base unit)
//! - `CENTI_FINTS`: 1,000 * MILLI_FINTS
//! - `FINTS`: 100 * CENTI_FINTS
//!
//! ## Time Constants
//!
//! - Block time: 3 seconds (3000 milliseconds)
//! - Session duration: 6 hours
//! - Era duration: 24 hours
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

pub mod currency {
    pub type Balance = u128;
    pub const MILLI_FINTS: Balance = 1_000_000_000;
    pub const CENTI_FINTS: Balance = 1_000 * MILLI_FINTS; // assume this is worth about a cent.
    pub const FINTS: Balance = 100 * CENTI_FINTS;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        //items as Balance * 15 * CENTI_FINTS + (bytes as Balance) * 6 * CENTI_FINTS
        items as Balance * 2 * CENTI_FINTS + (bytes as Balance) * (10 * MILLI_FINTS)
    }
}

/// Time.
pub mod time {
    pub type BlockNumber = u32;
    pub const MILLISECS_PER_BLOCK: u64 = 3000;
    pub const SECS_PER_BLOCK: u64 = MILLISECS_PER_BLOCK / 1000;
    // NOTE: Currently it is not possible to change the slot duration after the chain has started.
    //       Attempting to do so will brick block production.
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

    // NOTE: Currently it is not possible to change the epoch duration after the chain has started.
    //       Attempting to do so will brick block production.
    //For production make it 30-60 minutes per epoch after checking other parachains
    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
    const _ALLIANCE_MOTION_DURATION_IN_BLOCKS: BlockNumber = 5 * DAYS;
}
pub mod common {
    pub type Balance = u128;
    pub const DATA_DEPOSIT_PER_BYTE: Balance = crate::constants::currency::CENTI_FINTS;
    pub const MAXIMUM_REASON_LENGTH: u32 = 300;
}
