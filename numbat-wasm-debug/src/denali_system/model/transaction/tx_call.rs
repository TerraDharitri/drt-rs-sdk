use crate::{
    denali_system::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    DebugApi,
};
use numbat_wasm::types::{ContractCall, ContractCallWithRewa, DcdtTokenPayment};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::TxCallRaw,
};

use super::{tx_interpret_util::interpret_rewa_value, TxDCDT};

#[derive(Debug, Default)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub rewa_value: BigUintValue,
    pub dcdt_value: Vec<TxDCDT>,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            rewa_value: interpret_rewa_value(from.value, from.rewa_value, context),
            dcdt_value: from
                .dcdt_value
                .into_iter()
                .map(|dcdt_value| TxDCDT::interpret_from(dcdt_value, context))
                .collect(),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

impl IntoRaw<TxCallRaw> for TxCall {
    fn into_raw(self) -> TxCallRaw {
        TxCallRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            rewa_value: self.rewa_value.into_raw_opt(),
            dcdt_value: self
                .dcdt_value
                .into_iter()
                .map(|dcdt_value| dcdt_value.into_raw())
                .collect(),
            function: self.function,
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw(),
            gas_price: self.gas_price.into_raw(),
        }
    }
}

impl TxCall {
    pub fn to_contract_call(&self) -> ContractCallWithRewa<DebugApi, ()> {
        let mut contract_call = ContractCallWithRewa::new(
            (&self.to.value).into(),
            self.function.as_bytes(),
            (&self.rewa_value.value).into(),
        )
        .convert_to_dcdt_transfer_call(
            self.dcdt_value
                .iter()
                .map(|dcdt| {
                    DcdtTokenPayment::new(
                        dcdt.dcdt_token_identifier.value.as_slice().into(),
                        dcdt.nonce.value,
                        (&dcdt.dcdt_value.value).into(),
                    )
                })
                .collect(),
        );

        for argument in &self.arguments {
            contract_call.push_raw_argument(argument.value.as_slice());
        }
        contract_call
    }
}
