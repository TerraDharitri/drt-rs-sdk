use dharitri_sc_snippets::imports::*;
use forwarder_interact::forwarder_cli;

#[tokio::main]
async fn main() {
    forwarder_cli().await;
}
