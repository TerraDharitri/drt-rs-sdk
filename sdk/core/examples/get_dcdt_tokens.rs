use dharitri_sdk::{
    data::address::Address,
    gateway::{GatewayProxy, DEVNET_GATEWAY},
};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "drt1pdv0h3ddqyzlraek02y5rhmjnwwapjyhqm983kfcdfzmr6axqhds55z74c",
    )
    .unwrap();

    let blockchain = GatewayProxy::new(DEVNET_GATEWAY.to_string());
    let balances = blockchain.get_account_dcdt_tokens(&addr).await.unwrap();

    println!("{balances:#?}");
}
