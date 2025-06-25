use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BigUintValue, BytesValue, CheckValue, CheckValueList, U64Value},
    serde_raw::CheckDcdtInstanceRaw,
};

#[derive(Debug, Default)]
pub struct CheckDcdtInstance {
    pub nonce: U64Value,
    pub balance: CheckValue<BigUintValue>,
    pub creator: CheckValue<BytesValue>,
    pub royalties: CheckValue<U64Value>,
    pub hash: CheckValue<BytesValue>,
    pub uri: CheckValueList,
    pub attributes: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckDcdtInstanceRaw> for CheckDcdtInstance {
    fn interpret_from(from: CheckDcdtInstanceRaw, context: &InterpreterContext) -> Self {
        CheckDcdtInstance {
            nonce: U64Value::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            creator: CheckValue::<BytesValue>::interpret_from(from.creator, context),
            royalties: CheckValue::<U64Value>::interpret_from(from.royalties, context),
            hash: CheckValue::<BytesValue>::interpret_from(from.hash, context),
            uri: CheckValueList::interpret_from(from.uri, context),
            attributes: CheckValue::<BytesValue>::interpret_from(from.attributes, context),
        }
    }
}
