#![no_std]

extern crate alloc;

pub mod builtin_func_names;
pub mod types;

/// Re-exported for convenience.
pub use dharitri_sc_codec as codec;

/// The equivalent DCDT token identifier for transferring MOA, the native Dharitri token.
pub const MOA_000000_TOKEN_IDENTIFIER: &str = "MOA-000000";
