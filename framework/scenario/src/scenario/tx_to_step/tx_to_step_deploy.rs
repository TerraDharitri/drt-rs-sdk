use dharitri_sc::types::{
    Code, DeployCall, RHListExec, Tx, TxCodeValue, TxEnv, TxEnvWithTxHash, TxFromSpecified, TxGas,
    TxPayment,
};

use crate::scenario_model::{ScDeployStep, TxExpect, TxResponse};

use super::{address_annotated, code_annotated, gas_annotated, StepWrapper, TxToStep};

impl<Env, From, Payment, Gas, CodeValue, RH> TxToStep<Env, RH>
    for Tx<Env, From, (), Payment, Gas, DeployCall<Env, Code<CodeValue>>, RH>
where
    Env: TxEnvWithTxHash<RHExpect = TxExpect>,
    From: TxFromSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
    RH: RHListExec<TxResponse, Env>,
{
    type Step = ScDeployStep;

    fn tx_to_step(mut self) -> StepWrapper<Env, Self::Step, RH> {
        let mut step =
            tx_to_sc_deploy_step(&self.env, self.from, self.payment, self.gas, self.data);
        step.explicit_tx_hash = self.env.take_tx_hash();
        step.expect = Some(self.result_handler.list_preprocessing());

        StepWrapper {
            env: self.env,
            step,
            result_handler: self.result_handler,
        }
    }
}

pub fn tx_to_sc_deploy_step<Env, From, Payment, Gas, CodeValue>(
    env: &Env,
    from: From,
    payment: Payment,
    gas: Gas,
    data: DeployCall<Env, Code<CodeValue>>,
) -> ScDeployStep
where
    Env: TxEnv,
    From: TxFromSpecified<Env>,
    Payment: TxPayment<Env>,
    Gas: TxGas<Env>,
    CodeValue: TxCodeValue<Env>,
{
    let mut step = ScDeployStep::new()
        .from(address_annotated(env, &from))
        .code(code_annotated(env, data.code_source));
    step.tx.code_metadata = data.code_metadata;
    for arg in data.arg_buffer.iter_buffers() {
        step.tx.arguments.push(arg.to_vec().into());
    }

    step.tx.gas_limit = gas_annotated(env, gas);

    let full_payment_data = payment.into_full_payment_data(env);
    if let Some(annotated_rewa_payment) = full_payment_data.rewa {
        step.tx.rewa_value = annotated_rewa_payment.into();
    }

    step
}
