use denali::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckDcdtMapRaw,
};

use super::CheckDcdtMapContents;

#[derive(Debug)]
pub enum CheckDcdtMap {
    Unspecified,
    Star,
    Equal(CheckDcdtMapContents),
}

impl Default for CheckDcdtMap {
    fn default() -> Self {
        CheckDcdtMap::Unspecified
    }
}

impl CheckDcdtMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDcdtMap::Star)
    }
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

impl IntoRaw<CheckDcdtMapRaw> for CheckDcdtMap {
    fn into_raw(self) -> CheckDcdtMapRaw {
        match self {
            CheckDcdtMap::Unspecified => CheckDcdtMapRaw::Unspecified,
            CheckDcdtMap::Star => CheckDcdtMapRaw::Star,
            CheckDcdtMap::Equal(value) => CheckDcdtMapRaw::Equal(value.into_raw()),
        }
    }
}
