use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::CheckDcdtMapRaw,
};

use super::CheckDcdtMapContents;

#[derive(Debug)]
pub enum CheckDcdtMap {
    Unspecified,
    Star,
    Equal(CheckDcdtMapContents),
}

impl InterpretableFrom<CheckDcdtMapRaw> for CheckDcdtMap {
    fn interpret_from(from: CheckDcdtMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDcdtMapRaw::Unspecified => CheckDcdtMap::Unspecified,
            CheckDcdtMapRaw::Star => CheckDcdtMap::Star,
            CheckDcdtMapRaw::Equal(m) => {
                CheckDcdtMap::Equal(CheckDcdtMapContents::interpret_from(m, context))
            },
        }
    }
}

impl CheckDcdtMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDcdtMap::Star)
    }
}
