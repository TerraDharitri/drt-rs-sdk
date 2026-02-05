use dharitri_sdk::chain_core::std::Bech32Address;
use dharitri_sdk_http::{DEVNET_GATEWAY, GatewayHttpProxy};

#[tokio::main]
async fn main() {
    let addr = Bech32Address::from_bech32_string(
        "drt1pdv0h3ddqyzlraek02y5rhmjnwwapjyhqm983kfcdfzmr6axqhds55z74c".to_owned(),
    );

    let blockchain = GatewayHttpProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_dcdt_tokens(&addr).await.unwrap();

    assert!(!balances.is_empty());
    println!("{balances:#?}");
}
