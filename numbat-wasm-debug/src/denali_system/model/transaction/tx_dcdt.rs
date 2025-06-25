use crate::denali_system::model::{BigUintValue, BytesValue, U64Value};
use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::{TxDCDTRaw, ValueSubTree},
};

#[derive(Debug, Clone)]
pub struct TxDCDT {
    pub dcdt_token_identifier: BytesValue,
    pub nonce: U64Value,
    pub dcdt_value: BigUintValue,
}

impl InterpretableFrom<TxDCDTRaw> for TxDCDT {
    fn interpret_from(from: TxDCDTRaw, context: &InterpreterContext) -> Self {
        TxDCDT {
            dcdt_token_identifier: interpret_dcdt_token_identifier(from.token_identifier, context),
            nonce: interpret_opt_u64(from.nonce, context),
            dcdt_value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

impl IntoRaw<TxDCDTRaw> for TxDCDT {
    fn into_raw(self) -> TxDCDTRaw {
        TxDCDTRaw {
            token_identifier: Some(self.dcdt_token_identifier.into_raw()),
            nonce: self.nonce.into_raw_opt(),
            value: self.dcdt_value.into_raw(),
        }
    }
}

fn interpret_dcdt_token_identifier(
    dcdt_token_identifier: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BytesValue {
    if let Some(dcdt_token_identifier_raw) = dcdt_token_identifier {
        BytesValue::interpret_from(dcdt_token_identifier_raw, context)
    } else {
        BytesValue::empty()
    }
}

fn interpret_opt_u64(opt_u64: Option<ValueSubTree>, context: &InterpreterContext) -> U64Value {
    if let Some(u) = opt_u64 {
        U64Value::interpret_from(u, context)
    } else {
        U64Value::empty()
    }
}
