use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Account holds an Account's information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcdtBalance {
    pub token_identifier: String,
    pub balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdtBalanceData {
    pub dcdts: HashMap<String, DcdtBalance>,
}

// DcdtBalanceResponse holds the dcdt balance endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdtBalanceResponse {
    pub data: Option<DcdtBalanceData>,
    pub error: String,
    pub code: String,
}
