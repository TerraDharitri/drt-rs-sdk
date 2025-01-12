dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};
#[dharitri_sc::module]
pub trait CallPromisesBackTransfersModule: common::CommonModule {
    #[endpoint]
    fn forward_promise_retrieve_funds_back_transfers(
        &self,
        to: ManagedAddress,
        token: MoaOrDcdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let gas_limit = self.blockchain().get_gas_left() - 20_000_000;
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds(token, token_nonce, amount)
            .gas(gas_limit)
            .callback(self.callbacks().retrieve_funds_back_transfers_callback())
            .gas_for_callback(10_000_000)
            .register_promise();
    }

    #[promises_callback]
    fn retrieve_funds_back_transfers_callback(&self) {
        let back_transfers = self.blockchain().get_back_transfers();
        let moa_transfer = back_transfers.total_moa_amount;

        if moa_transfer != BigUint::zero() {
            let moa_token_id = MoaOrDcdtTokenIdentifier::moa();
            self.retrieve_funds_callback_event(&moa_token_id, 0, &moa_transfer);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: moa_token_id,
                token_nonce: 0,
                token_amount: moa_transfer,
                args: ManagedVec::new(),
            });
        }

        for dcdt_transfer in &back_transfers.dcdt_payments {
            let dcdt_token_id =
                MoaOrDcdtTokenIdentifier::dcdt(dcdt_transfer.token_identifier.clone());
            self.retrieve_funds_callback_event(
                &dcdt_token_id,
                dcdt_transfer.token_nonce,
                &dcdt_transfer.amount,
            );

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: dcdt_token_id,
                token_nonce: dcdt_transfer.token_nonce,
                token_amount: dcdt_transfer.amount.clone(),
                args: ManagedVec::new(),
            });
        }
    }
}
