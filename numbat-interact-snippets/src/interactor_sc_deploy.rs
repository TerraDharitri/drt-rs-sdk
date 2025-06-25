use crate::{denali_to_drtrs_address, Interactor, InteractorResult};
use numbat_sdk_drtrs::data::{address::Address as DrtrsAddress, transaction::Transaction};
use numbat_wasm_debug::{
    numbat_wasm::numbat_codec::{CodecFrom, TopEncodeMulti},
    denali_system::model::{ScDeployStep, TypedScDeploy},
};
use log::info;

const DEPLOY_RECEIVER: [u8; 32] = [0u8; 32];
impl Interactor {
    fn sc_deploy_to_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.rewa_value.value.to_string(),
            sender: denali_to_drtrs_address(&sc_deploy_step.tx.from),
            receiver: DrtrsAddress::from_bytes(DEPLOY_RECEIVER),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64::encode(sc_deploy_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub async fn send_sc_deploy(&mut self, sc_call_step: ScDeployStep) -> String {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.sc_deploy_to_tx(&sc_call_step);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        self.proxy.send_transaction(&transaction).await.unwrap()
    }

    pub async fn sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> InteractorResult<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_call_step: ScDeployStep = typed_sc_call.into();
        let tx_hash = self.send_sc_deploy(sc_call_step).await;
        println!("deploy tx hash: {tx_hash}");
        info!("deploy tx hash: {}", tx_hash);
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        InteractorResult::new(tx)
    }
}
