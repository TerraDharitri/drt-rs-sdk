use dharitri_sdk::chain_core::std::Bech32Address;
use dharitri_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let addr = Bech32Address::from_bech32_string(
        "drt1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ssj7kxgw".to_owned(),
    );

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let account = blockchain.get_account(&addr).await.unwrap();

    println!("account: {account:#?}");
}
