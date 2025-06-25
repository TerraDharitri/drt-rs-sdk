use std::time::Duration;

#[allow(unused_imports)]
use dharitri_sc_snippets::dharitri_sc::types::{
    DcdtTokenPayment, MultiValueEncoded, TokenIdentifier,
};
use dharitri_sc_snippets::{
    dharitri_sc::types::{ContractCall, ContractCallNoPayment},
    dharitri_sc_scenario::{
        denali_system::ScenarioRunner, scenario_format::interpret_trait::InterpretableFrom,
        standalone::retrieve_account_as_scenario_set_state,
    },
};

use super::*;

const WREWA_SWAP_SC_BECH32: &str = "drt1qqqqqqqqqqqqqpgqcy2wua5cq59y6sxqj2ka3scayh5e5ms7cthq2hs9gl";
const WREWA_TOKEN_IDENTIFIER: &str = "WREWA-6cf38e";
const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 REWA
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WREWA

impl MultisigInteract {
    pub async fn wrewa_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_rewa().await;
        self.wrap_rewa().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_rewa().await;
    }

    pub async fn wrap_rewa(&mut self) {
        println!("proposing wrap rewa...");
        let action_id = self.propose_wrap_rewa().await;

        println!("perfoming wrap rewa action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn unwrap_rewa(&mut self) {
        println!("proposing unwrap rewa...");
        let action_id = self.propose_unwrap_rewa().await;

        println!("perfoming unwrap rewa action `{action_id}`...");
        self.perform_action(action_id, "15,000,000").await;
    }

    pub async fn wrewa_swap_set_state(&mut self) {
        let scenario_raw = retrieve_account_as_scenario_set_state(
            Config::load_config().gateway().to_string(),
            WREWA_SWAP_SC_BECH32.to_string(),
            true,
        )
        .await;

        let scenario = Scenario::interpret_from(scenario_raw, &InterpreterContext::default());

        self.interactor.pre_runners.run_scenario(&scenario);
        self.interactor.post_runners.run_scenario(&scenario);
    }

    async fn propose_wrap_rewa(&mut self) -> usize {
        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        bech32::decode(WREWA_SWAP_SC_BECH32),
                        WRAP_AMOUNT,
                        "wrapRewa".to_string(),
                        MultiValueEncoded::new(),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed wrap rewa action `{action_id}`");
        action_id
    }

    async fn propose_unwrap_rewa(&mut self) -> usize {
        let contract_call = ContractCallNoPayment::<StaticApi, ()>::new(
            bech32::decode(WREWA_SWAP_SC_BECH32).into(),
            "unwrapRewa",
        )
        .with_dcdt_transfer(DcdtTokenPayment::new(
            TokenIdentifier::from(WREWA_TOKEN_IDENTIFIER),
            0u64,
            UNWRAP_AMOUNT.into(),
        ))
        .into_normalized();

        let action_id = self
            .interactor
            .sc_call_get_result(
                ScCallStep::new()
                    .call(self.state.multisig().propose_async_call(
                        contract_call.basic.to,
                        0u64,
                        contract_call.basic.endpoint_name,
                        contract_call.basic.arg_buffer.into_multi_value_encoded(),
                    ))
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000"),
            )
            .await
            .result
            .unwrap();

        println!("successfully proposed unwrap rewa action `{action_id}`");
        action_id
    }
}
