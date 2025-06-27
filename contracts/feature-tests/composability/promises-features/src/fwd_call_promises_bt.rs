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
        token: RewaOrDcdtTokenIdentifier,
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
        let rewa_transfer = back_transfers.total_rewa_amount;

        if rewa_transfer != BigUint::zero() {
            let rewa_token_id = RewaOrDcdtTokenIdentifier::rewa();
            self.retrieve_funds_callback_event(&rewa_token_id, 0, &rewa_transfer);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: rewa_token_id,
                token_nonce: 0,
                token_amount: rewa_transfer,
                args: ManagedVec::new(),
            });
        }

        for dcdt_transfer in &back_transfers.dcdt_payments {
            let (token, nonce, payment) = dcdt_transfer.into_tuple();
            let dcdt_token_id = RewaOrDcdtTokenIdentifier::dcdt(token);
            self.retrieve_funds_callback_event(&dcdt_token_id, nonce, &payment);

            let _ = self.callback_data().push(&CallbackData {
                callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
                token_identifier: dcdt_token_id,
                token_nonce: nonce,
                token_amount: payment,
                args: ManagedVec::new(),
            });
        }
    }
}
