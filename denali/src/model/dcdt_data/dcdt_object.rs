use crate::model::{BytesValue, U64Value};

use super::DcdtInstance;

#[derive(Debug, Default)]
pub struct DcdtObject {
    pub token_identifier: Option<BytesValue>,
    pub instances: Vec<DcdtInstance>,
    pub last_nonce: Option<U64Value>,
    pub roles: Vec<BytesValue>,
    pub frozen: Option<U64Value>,
}
