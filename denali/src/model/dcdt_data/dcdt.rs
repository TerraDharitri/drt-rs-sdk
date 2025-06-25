use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesKey, BytesValue, U64Value},
    serde_raw::DcdtRaw,
};

use super::{DcdtInstance, DcdtObject};

#[derive(Debug)]
pub enum Dcdt {
    Short(BytesKey),
    Full(DcdtObject),
}

impl InterpretableFrom<DcdtRaw> for Dcdt {
    fn interpret_from(from: DcdtRaw, context: &InterpreterContext) -> Self {
        match from {
            DcdtRaw::Short(short_dcdt) => {
                Dcdt::Short(BytesKey::interpret_from(short_dcdt, context))
            },
            DcdtRaw::Full(full_dcdt) => Dcdt::Full(DcdtObject {
                token_identifier: full_dcdt
                    .token_identifier
                    .map(|b| BytesValue::interpret_from(b, context)),
                instances: full_dcdt
                    .instances
                    .into_iter()
                    .map(|instance| DcdtInstance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_dcdt
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_dcdt
                    .roles
                    .into_iter()
                    .map(|role| BytesValue::interpret_from(role, context))
                    .collect(),
                frozen: full_dcdt
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}
