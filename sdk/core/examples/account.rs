use dharitri_sdk::{
    data::address::Address,
    gateway::{GatewayProxy, DEVNET_GATEWAY},
};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "drt1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ssj7kxgw",
    )
    .unwrap();

    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let account = blockchain.get_account(&addr).await.unwrap();

    println!("account: {account:#?}");
}
