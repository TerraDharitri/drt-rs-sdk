use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::BytesKey,
    serde_raw::CheckDcdtRaw,
};

use super::CheckDcdtData;

#[derive(Debug)]
pub enum CheckDcdt {
    Short(BytesKey),
    Full(CheckDcdtData),
}

impl InterpretableFrom<CheckDcdtRaw> for CheckDcdt {
    fn interpret_from(from: CheckDcdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDcdtRaw::Full(m) => CheckDcdt::Full(CheckDcdtData::interpret_from(m, context)),
            CheckDcdtRaw::Short(v) => CheckDcdt::Short(BytesKey::interpret_from(v, context)),
        }
    }
}
