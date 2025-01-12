use std::time::Duration;

use dharitri_sc_snippets::imports::*;

use super::*;

const WRAP_AMOUNT: u64 = 50000000000000000; // 0.05 MOA
const UNWRAP_AMOUNT: u64 = 25000000000000000; // 0.025 WMOA

impl MultisigInteract {
    pub async fn wmoa_swap_full(&mut self) {
        self.deploy().await;
        self.feed_contract_moa().await;
        self.wrap_moa().await;
        self.interactor.sleep(Duration::from_secs(15)).await;
        self.unwrap_moa().await;
    }

    pub async fn wrap_moa(&mut self) {
        println!("proposing wrap moa...");
        let action_id = self.propose_wrap_moa().await;

        println!("perfoming wrap moa action `{action_id}`...");
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn unwrap_moa(&mut self) {
        println!("proposing unwrap moa...");
        let action_id = self.propose_unwrap_moa().await;

        println!("perfoming unwrap moa action `{action_id}`...");
        self.perform_action(action_id, 15_000_000u64).await;
    }

    pub async fn wmoa_swap_set_state(&mut self) {
        self.interactor
            .retrieve_account(&self.config.wmoa_address)
            .await;
    }

    async fn propose_wrap_moa(&mut self) -> usize {
        let function_call = self
            .interactor
            .tx()
            .to(&self.config.wmoa_address)
            .typed(wmoa_proxy::MoaDcdtSwapProxy)
            .wrap_moa()
            .into_function_call();

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(&self.config.wmoa_address, WRAP_AMOUNT, function_call)
            .returns(ReturnsResult)
            .run()
            .await;

        println!("successfully proposed wrap moa action `{action_id}`");
        action_id
    }

    pub async fn query_wmoa_token_identifier(&mut self) -> TokenIdentifier<StaticApi> {
        let wmoa_token_id = self
            .interactor
            .query()
            .to(&self.config.wmoa_address)
            .typed(wmoa_proxy::MoaDcdtSwapProxy)
            .wrapped_moa_token_id()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("WMOA token identifier: {wmoa_token_id}");

        wmoa_token_id
    }

    async fn propose_unwrap_moa(&mut self) -> usize {
        let wmoa_token_id = self.query_wmoa_token_identifier().await;

        let normalized_tx = self
            .interactor
            .tx()
            .to(&self.config.wmoa_address)
            .typed(wmoa_proxy::MoaDcdtSwapProxy)
            .unwrap_moa()
            .single_dcdt(&wmoa_token_id, 0u64, &UNWRAP_AMOUNT.into())
            .normalize();
        let normalized_to = normalized_tx.to;
        let normalized_data = normalized_tx.data;

        let action_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_multisig_address())
            .gas(NumExpr("10,000,000"))
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(normalized_to, 0u64, normalized_data)
            .returns(ReturnsResult)
            .run()
            .await;

        println!("successfully proposed unwrap moa action `{action_id}`");
        action_id
    }
}
