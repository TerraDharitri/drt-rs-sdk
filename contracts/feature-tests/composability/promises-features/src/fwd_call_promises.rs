dharitri_sc::imports!();

use crate::{
    common::{self, CallbackData},
    vault_proxy,
};

#[dharitri_sc::module]
pub trait CallPromisesModule: common::CommonModule {
    #[endpoint]
    #[payable("*")]
    fn forward_promise_accept_funds(&self, to: ManagedAddress) {
        let payment = self.call_value().moa_or_single_dcdt();
        let gas_limit = self.blockchain().get_gas_left() / 2;

        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .accept_funds()
            .gas(gas_limit)
            .payment(payment)
            .register_promise();
    }

    #[endpoint]
    fn forward_promise_retrieve_funds(
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
            .callback(self.callbacks().retrieve_funds_callback())
            .gas_for_callback(10_000_000)
            .register_promise();
    }

    #[promises_callback]
    fn retrieve_funds_callback(&self) {
        let (token, nonce, payment) = self.call_value().moa_or_single_dcdt().into_tuple();
        self.retrieve_funds_callback_event(&token, nonce, &payment);

        let _ = self.callback_data().push(&CallbackData {
            callback_name: ManagedBuffer::from(b"retrieve_funds_callback"),
            token_identifier: token,
            token_nonce: nonce,
            token_amount: payment,
            args: ManagedVec::new(),
        });
    }

    #[endpoint]
    #[payable("*")]
    fn forward_payment_callback(&self, to: ManagedAddress) {
        let payment = self.call_value().any_payment();
        let gas_limit = self.blockchain().get_gas_left() / 2;

        self.tx()
            .to(&to)
            .gas(gas_limit)
            .payment(payment)
            .callback(self.callbacks().transfer_callback())
            .register_promise();
    }

    #[promises_callback]
    fn transfer_callback(&self, #[call_result] result: MultiValueEncoded<ManagedBuffer>) {
        self.callback_result(result);

        let call_value = self.call_value().any_payment();
        match call_value {
            MoaOrMultiDcdtPayment::Moa(moa) => {
                self.retrieve_funds_callback_event(&MoaOrDcdtTokenIdentifier::moa(), 0, &moa);
                let _ = self.callback_data().push(&CallbackData {
                    callback_name: ManagedBuffer::from(b"transfer_callback"),
                    token_identifier: MoaOrDcdtTokenIdentifier::moa(),
                    token_nonce: 0,
                    token_amount: moa,
                    args: ManagedVec::new(),
                });
            },
            MoaOrMultiDcdtPayment::MultiDcdt(multi_dcdt) => {
                for dcdt in multi_dcdt.into_iter() {
                    let token_identifier = MoaOrDcdtTokenIdentifier::dcdt(dcdt.token_identifier);
                    self.retrieve_funds_callback_event(&token_identifier, 0, &dcdt.amount);
                    let _ = self.callback_data().push(&CallbackData {
                        callback_name: ManagedBuffer::from(b"transfer_callback"),
                        token_identifier,
                        token_nonce: 0,
                        token_amount: dcdt.amount,
                        args: ManagedVec::new(),
                    });
                }
            },
        }
    }
}