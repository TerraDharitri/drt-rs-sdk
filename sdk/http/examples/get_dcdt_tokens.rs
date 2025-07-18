use dharitri_sdk::data::sdk_address::SdkAddress;
use dharitri_sdk_http::{GatewayHttpProxy, DEVNET_GATEWAY};

#[tokio::main]
async fn main() {
    let addr = SdkAddress::from_bech32_string(
        "drt1pdv0h3ddqyzlraek02y5rhmjnwwapjyhqm983kfcdfzmr6axqhds55z74c",
    )
    .unwrap();

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_dcdt_tokens(&addr.0).await.unwrap();

    assert!(!balances.is_empty());
    println!("{balances:#?}");
}
