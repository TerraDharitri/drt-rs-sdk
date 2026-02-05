use crate::data::dcdt::{DcdtBalance, DcdtBalanceResponse};
use anyhow::anyhow;
use dharitri_chain_core::std::Bech32Address;
use std::collections::HashMap;

use super::{ACCOUNT_ENDPOINT, GatewayRequest, GatewayRequestType};

/// Retrieves an all dcdt tokens of an account from the network.
pub struct GetAccountDcdtTokensRequest<'a> {
    pub address: &'a Bech32Address,
}

impl<'a> GetAccountDcdtTokensRequest<'a> {
    pub fn new(address: &'a Bech32Address) -> Self {
        Self { address }
    }
}

impl GatewayRequest for GetAccountDcdtTokensRequest<'_> {
    type Payload = ();
    type DecodedJson = DcdtBalanceResponse;
    type Result = HashMap<String, DcdtBalance>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}/{}/dcdt", self.address.bech32)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.dcdts),
        }
    }
}
