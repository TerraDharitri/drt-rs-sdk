use alloc::vec::Vec;
use numbat_wasm::types::Address;
use num_bigint::BigUint;
use std::{collections::HashMap, fmt, fmt::Write};

use crate::key_hex;

use super::AccountDcdt;

pub type AccountStorage = HashMap<Vec<u8>, Vec<u8>>;

#[derive(Clone, Debug)]
pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub rewa_balance: BigUint,
    pub dcdt: AccountDcdt,
    pub storage: AccountStorage,
    pub username: Vec<u8>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<Address>,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut storage_buf = String::new();
        let mut storage_keys: Vec<Vec<u8>> = self.storage.iter().map(|(k, _)| k.clone()).collect();
        storage_keys.sort();

        for key in &storage_keys {
            let value = self.storage.get(key).unwrap();
            write!(
                storage_buf,
                "\n\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )
            .unwrap();
        }

        write!(
            f,
            "AccountData {{
		nonce: {},
		balance: {},
		dcdt: [{} ],
		username: {},
		storage: [{} ]
	}}",
            self.nonce,
            self.rewa_balance,
            self.dcdt,
            String::from_utf8(self.username.clone()).unwrap(),
            storage_buf
        )
    }
}
