use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum CheckDcdt {
    Star,
    Equal(BTreeMap<BytesKey, CheckValue<BigUintValue>>),
}

impl InterpretableFrom<CheckDcdtRaw> for CheckDcdt {
    fn interpret_from(from: CheckDcdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDcdtRaw::Unspecified => CheckDcdt::Equal(BTreeMap::new()),
            CheckDcdtRaw::Star => CheckDcdt::Star,
            CheckDcdtRaw::Equal(m) => CheckDcdt::Equal(
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            BytesKey::interpret_from(k, context),
                            CheckValue::<BigUintValue>::interpret_from(v, context),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl CheckDcdt {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDcdt::Star)
    }
}
