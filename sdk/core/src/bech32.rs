//! Duplicated between sdk and scenarios.
//!
//! TODO: de-duplicate, place in chain core crate, if possible.

use bech32::{Bech32, Hrp};
use dharitri_chain_core::types::Address;

pub fn decode(bech32_address: &str) -> Address {
    let (_hrp, dest_address_bytes) = bech32::decode(bech32_address)
        .unwrap_or_else(|err| panic!("bech32 decode error for {bech32_address}: {err}"));
    if dest_address_bytes.len() != 32 {
        panic!("Invalid address length after decoding")
    }

    Address::from_slice(&dest_address_bytes)
}

pub fn encode(address: &Address) -> String {
    let hrp = Hrp::parse("drt").expect("invalid hrp");
    bech32::encode::<Bech32>(hrp, address.as_bytes()).expect("bech32 encode error")
}
