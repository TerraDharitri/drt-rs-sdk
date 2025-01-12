extern crate ping_pong_moa_interact;

#[tokio::main]
pub async fn main() {
    ping_pong_moa_interact::ping_pong_moa_cli().await;
}
