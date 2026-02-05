use crate::data::dcdt::DcdtRolesResponse;
use anyhow::anyhow;
use dharitri_chain_core::std::Bech32Address;
use std::collections::HashMap;

use super::{ACCOUNT_ENDPOINT, GatewayRequest, GatewayRequestType};

/// Retrieves an all dcdt roles of an account from the network.
pub struct GetAccountDcdtRolesRequest<'a> {
    pub address: &'a Bech32Address,
}

impl<'a> GetAccountDcdtRolesRequest<'a> {
    pub fn new(address: &'a Bech32Address) -> Self {
        Self { address }
    }
}

impl GatewayRequest for GetAccountDcdtRolesRequest<'_> {
    type Payload = ();
    type DecodedJson = DcdtRolesResponse;
    type Result = HashMap<String, Vec<String>>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}/{}/dcdts/roles", self.address.bech32)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.roles),
        }
    }
}
