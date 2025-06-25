use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue},
    serde_raw::TxValidatorRewardRaw,
};

use super::tx_interpret_util::interpret_rewa_value;

#[derive(Debug)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub rewa_value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            rewa_value: interpret_rewa_value(from.value, from.rewa_value, context),
        }
    }
}
