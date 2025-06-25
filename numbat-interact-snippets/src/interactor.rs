use numbat_sdk_drtrs::{
    blockchain::rpc::NumbatProxy,
    data::{address::Address as DrtrsAddress, network_config::NetworkConfig},
    interactors::wallet::Wallet,
};
use numbat_wasm_debug::{numbat_wasm::types::Address, denali_system::model::AddressValue, HashMap};
use std::time::Duration;

use crate::Sender;

pub struct Interactor {
    pub proxy: NumbatProxy,
    pub network_config: NetworkConfig,
    pub sender_map: HashMap<Address, Sender>,

    pub(crate) waiting_time_ms: u64,
}

impl Interactor {
    pub async fn new(gateway_url: &str) -> Self {
        let proxy = NumbatProxy::new(gateway_url.to_string());
        let network_config = proxy.get_network_config().await.unwrap();
        Self {
            proxy,
            network_config,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
        }
    }

    pub fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = drtrs_address_to_h256(wallet.address());
        self.sender_map.insert(
            address.clone(),
            Sender {
                address: address.clone(),
                wallet,
                current_nonce: None,
            },
        );
        address
    }

    pub async fn sleep(&mut self, duration: Duration) {
        self.waiting_time_ms += duration.as_millis() as u64;
        tokio::time::sleep(duration).await;
    }
}

pub(crate) fn denali_to_drtrs_address(denali_address: &AddressValue) -> DrtrsAddress {
    let bytes = denali_address.value.as_array();
    DrtrsAddress::from_bytes(*bytes)
}

pub(crate) fn address_h256_to_drtrs(address: &Address) -> DrtrsAddress {
    let bytes = address.as_array();
    DrtrsAddress::from_bytes(*bytes)
}

pub(crate) fn drtrs_address_to_h256(drtrs_address: DrtrsAddress) -> Address {
    drtrs_address.to_bytes().into()
}
