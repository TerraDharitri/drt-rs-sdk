use numbat_wasm::types::Address;
use num_bigint::BigUint;

use crate::{tx_mock::TxPanic, world_mock::DcdtInstanceMetadata};

use super::TxCache;

impl TxCache {
    pub fn subtract_rewa_balance(&self, address: &Address, call_value: &BigUint) {
        self.with_account_mut(address, |account| {
            assert!(
                &account.rewa_balance >= call_value,
                "failed transfer (insufficient funds)"
            );
            account.rewa_balance -= call_value;
        })
    }

    pub fn subtract_tx_gas(&self, address: &Address, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.rewa_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.rewa_balance -= &gas_cost;
        });
    }

    pub fn increase_rewa_balance(&self, address: &Address, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.rewa_balance += amount;
        });
    }

    #[allow(clippy::redundant_closure)] // clippy is wrong here, `.unwrap_or_else(panic_insufficient_funds)` won't compile
    pub fn subtract_dcdt_balance(
        &self,
        address: &Address,
        dcdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> DcdtInstanceMetadata {
        self.with_account_mut(address, |account| {
            let dcdt_data_map = &mut account.dcdt;
            let dcdt_data = dcdt_data_map
                .get_mut_by_identifier(dcdt_token_identifier)
                .unwrap_or_else(|| panic_insufficient_funds());

            let dcdt_instances = &mut dcdt_data.instances;
            let dcdt_instance = dcdt_instances
                .get_mut_by_nonce(nonce)
                .unwrap_or_else(|| panic_insufficient_funds());
            let dcdt_balance = &mut dcdt_instance.balance;
            if &*dcdt_balance < value {
                panic_insufficient_funds();
            }

            *dcdt_balance -= value;

            dcdt_instance.metadata.clone()
        })
    }

    pub fn increase_dcdt_balance(
        &self,
        address: &Address,
        dcdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
        dcdt_metadata: DcdtInstanceMetadata,
    ) {
        self.with_account_mut(address, |account| {
            account.dcdt.increase_balance(
                dcdt_token_identifier.to_vec(),
                nonce,
                value,
                dcdt_metadata,
            );
        });
    }

    pub fn transfer_dcdt_balance(
        &self,
        from: &Address,
        to: &Address,
        dcdt_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        let metadata = self.subtract_dcdt_balance(from, dcdt_token_identifier, nonce, value);

        self.increase_dcdt_balance(to, dcdt_token_identifier, nonce, value, metadata);
    }
}

fn panic_insufficient_funds() -> ! {
    std::panic::panic_any(TxPanic {
        status: 10,
        message: b"insufficient funds".to_vec(),
    });
}
