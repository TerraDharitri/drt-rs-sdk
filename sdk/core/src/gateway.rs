mod gateway_account;
mod gateway_block;
mod gateway_network;
mod gateway_proxy;
mod gateway_tx;
mod gateway_tx_retrieve;

pub use gateway_proxy::GatewayProxy;

pub const MAINNET_GATEWAY: &str = "https://gateway.dharitri.org";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.dharitri.org";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.dharitri.org";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;
