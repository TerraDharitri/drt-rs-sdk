use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue, U64Value},
    serde_raw::TxTransferRaw,
};

use super::{tx_interpret_util::interpret_rewa_value, TxDCDT};

#[derive(Debug)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub rewa_value: BigUintValue,
    pub dcdt_value: Vec<TxDCDT>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            rewa_value: interpret_rewa_value(from.value, from.rewa_value, context),
            dcdt_value: from
                .dcdt_value
                .iter()
                .map(|dcdt_value| TxDCDT::interpret_from(dcdt_value.clone(), context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit.unwrap_or_default(), context),
            gas_price: U64Value::interpret_from(from.gas_price.unwrap_or_default(), context),
        }
    }
}
