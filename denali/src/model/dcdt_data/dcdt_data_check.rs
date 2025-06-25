use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{CheckValue, U64Value},
    serde_raw::CheckDcdtDataRaw,
};

use super::CheckDcdtInstances;

#[derive(Debug, Default)]
pub struct CheckDcdtData {
    pub instances: CheckDcdtInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckDcdtDataRaw> for CheckDcdtData {
    fn interpret_from(from: CheckDcdtDataRaw, context: &InterpreterContext) -> Self {
        CheckDcdtData {
            instances: CheckDcdtInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}
