# Numbat SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/numbat-drtrs-sdk)](https://crates.io/crates/numbat-drtrs-sdk)

## Example

```rust
use numbat_sdk_drtrs::blockchain::rpc::{NumbatProxy, DEVNET_GATEWAY};

#[tokio::test]
async fn get_network_config() {
    let blockchain = NumbatProxy::new(DEVNET_GATEWAY.to_string());
    let network_config = blockchain.get_network_config().await.unwrap();

    println!("network_config: {:?}", network_config)
}
```

More example in `./src/blockchain/tests.rs`