use dharitri_sc_scenario::{
    imports::{Bech32Address, ScenarioRunner},
    denali_system::{run_list::ScenarioRunnerList, run_trace::ScenarioTraceFile},
    dharitri_sc::types::Address,
    scenario_model::AddressValue,
};
use dharitri_sdk::{
    data::{address::Address as DrtrsAddress, network_config::NetworkConfig},
    gateway::GatewayProxy,
    wallet::Wallet,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{account_tool::retrieve_account_as_scenario_set_state, Sender};

pub const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

pub struct Interactor {
    pub proxy: GatewayProxy,
    pub network_config: NetworkConfig,
    pub sender_map: HashMap<Address, Sender>,

    pub(crate) waiting_time_ms: u64,
    pub pre_runners: ScenarioRunnerList,
    pub post_runners: ScenarioRunnerList,

    pub current_dir: PathBuf,
}

impl Interactor {
    pub async fn new(gateway_url: &str) -> Self {
        let proxy = GatewayProxy::new(gateway_url.to_string());
        let network_config = proxy.get_network_config().await.unwrap();
        Self {
            proxy,
            network_config,
            sender_map: HashMap::new(),
            waiting_time_ms: 0,
            pre_runners: ScenarioRunnerList::empty(),
            post_runners: ScenarioRunnerList::empty(),
            current_dir: PathBuf::default(),
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

    pub async fn with_tracer<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.post_runners.push(ScenarioTraceFile::new(path));
        self
    }

    pub async fn retrieve_account(&mut self, wallet_address: &Bech32Address) {
        let set_state = retrieve_account_as_scenario_set_state(&self.proxy, wallet_address).await;
        self.pre_runners.run_set_state_step(&set_state);
        self.post_runners.run_set_state_step(&set_state);
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
