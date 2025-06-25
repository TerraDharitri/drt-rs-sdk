use num_bigint::BigUint;
use num_traits::Zero;

use crate::key_hex;
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{DcdtInstanceMetadata, DcdtInstances, DcdtRoles};

#[derive(Clone, Default, Debug)]
pub struct DcdtData {
    pub token_identifier: Vec<u8>,
    pub instances: DcdtInstances,
    pub last_nonce: u64,
    pub roles: DcdtRoles,
    pub frozen: bool,
}

impl DcdtData {
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty_dcdt()
            && self.last_nonce == 0
            && self.roles.is_empty()
            && !self.frozen
    }

    pub fn get_roles(&self) -> Vec<Vec<u8>> {
        self.roles.get()
    }
}

#[derive(Clone, Default, Debug)]
pub struct AccountDcdt(HashMap<Vec<u8>, DcdtData>);

impl AccountDcdt {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_identifier(&self, identifier: &[u8]) -> Option<&DcdtData> {
        self.0.get(identifier)
    }

    /// Will provide a clone.
    pub fn get_roles(&self, identifier: &[u8]) -> Vec<Vec<u8>> {
        self.get_by_identifier_or_default(identifier).get_roles()
    }

    pub fn set_roles(&mut self, token_identifier: Vec<u8>, roles: Vec<Vec<u8>>) {
        let dcdt_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DcdtData {
                token_identifier,
                instances: DcdtInstances::new(),
                last_nonce: 0,
                roles: DcdtRoles::default(),
                frozen: false,
            });
        dcdt_data.roles = DcdtRoles::new(roles);
    }

    /// Will provide a clone.
    pub fn get_by_identifier_or_default(&self, identifier: &[u8]) -> DcdtData {
        if let Some(value) = self.0.get(identifier) {
            value.clone()
        } else {
            DcdtData::default()
        }
    }

    pub fn get_mut_by_identifier(&mut self, identifier: &[u8]) -> Option<&mut DcdtData> {
        self.0.get_mut(identifier)
    }

    pub fn new_from_raw_map(hash: HashMap<Vec<u8>, DcdtData>) -> Self {
        AccountDcdt(hash)
    }

    pub fn increase_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: DcdtInstanceMetadata,
    ) {
        let dcdt_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DcdtData {
                token_identifier,
                instances: DcdtInstances::new(),
                last_nonce: nonce,
                roles: DcdtRoles::default(),
                frozen: false,
            });
        dcdt_data.instances.increase_balance(nonce, value, metadata);
    }

    pub fn set_dcdt_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: DcdtInstanceMetadata,
    ) {
        let dcdt_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DcdtData {
                token_identifier,
                instances: DcdtInstances::new(),
                last_nonce: nonce,
                roles: DcdtRoles::default(),
                frozen: false,
            });
        dcdt_data.instances.set_balance(nonce, value, metadata);
    }

    pub fn get_dcdt_balance(&self, token_identifier: &[u8], nonce: u64) -> BigUint {
        if let Some(dcdt_data) = self.get_by_identifier(token_identifier) {
            if let Some(instance) = dcdt_data.instances.get_by_nonce(nonce) {
                instance.balance.clone()
            } else {
                BigUint::zero()
            }
        } else {
            BigUint::zero()
        }
    }

    pub fn add_uris(&mut self, token_identifier: &[u8], nonce: u64, mut new_uris: Vec<Vec<u8>>) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .uri
            .append(&mut new_uris);
    }

    pub fn update_attributes(
        &mut self,
        token_identifier: &[u8],
        nonce: u64,
        new_attribute_bytes: Vec<u8>,
    ) {
        self.0
            .get_mut(token_identifier)
            .unwrap_or_else(|| panic!("invalid token"))
            .instances
            .get_mut_by_nonce(nonce)
            .unwrap_or_else(|| panic!("invalid token nonce"))
            .metadata
            .attributes = new_attribute_bytes;
    }

    pub fn iter(&self) -> Iter<Vec<u8>, DcdtData> {
        self.0.iter()
    }
}

impl fmt::Display for DcdtData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dcdt_buf = String::new();
        write!(
            dcdt_buf,
            "{{
                token_identifier: {},
                instances: [{}],
                last_nonce: {},
                roles: [{}],
                frozen: {},
            }}",
            key_hex(self.token_identifier.as_slice()),
            self.instances,
            self.last_nonce,
            self.roles,
            self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountDcdt {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dcdt_buf = String::new();
        let dcdt_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &dcdt_keys {
            let value = self.0.get(key).unwrap();
            write!(dcdt_buf, "\n\t\t\t{} -> {}", key_hex(key.as_slice()), value)?;
        }
        Ok(())
    }
}
