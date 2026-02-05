use crate::vault_proxy;

dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait BackTransfersModule {
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<PaymentMultiValue>,
    ) {
        let bt_multi = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let rewa_sum = bt_multi.rewa_sum();
        if rewa_sum > 0u32 {
            self.back_transfers_rewa_event(rewa_sum);
        }
        self.back_transfers_multi_event(bt_multi.into_payment_vec().into_multi_value());

        let mut balances_after = MultiValueEncoded::new();
        for transfer in transfers {
            let payment = transfer.into_inner();
            let balance = self
                .blockchain()
                .get_sc_balance(&payment.token_identifier, payment.token_nonce);
            balances_after.push(MultiValue3::from((
                payment.token_identifier,
                payment.token_nonce,
                balance,
            )));
        }
        self.balances_after(balances_after);
    }

    /// Highlights the behavior when calling back transfers **without** reset.
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_twice(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<PaymentMultiValue>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .sync_call();

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfers)
            .sync_call();

        self.back_transfers_multi_event(back_transfers.into_payment_vec().into_multi_value());
    }

    /// Highlights the behavior when calling back transfers **with** reset.
    #[endpoint]
    fn forward_sync_retrieve_funds_bt_multi_twice_reset(
        &self,
        to: ManagedAddress,
        transfers: MultiValueEncoded<PaymentMultiValue>,
    ) {
        self.tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .sync_call();

        let back_transfers = self
            .tx()
            .to(&to)
            .typed(vault_proxy::VaultProxy)
            .retrieve_funds_multi(&transfers)
            .returns(ReturnsBackTransfersReset)
            .sync_call();

        self.back_transfers_multi_event(back_transfers.into_payment_vec().into_multi_value());
    }

    #[event("back_transfers_multi_event")]
    fn back_transfers_multi_event(
        &self,
        #[indexed] back_transfers: MultiValueEncoded<PaymentMultiValue>,
    );

    #[event("back_transfers_rewa_event")]
    fn back_transfers_rewa_event(&self, #[indexed] rewa_value: BigUint);

    #[event]
    fn balances_after(
        &self,
        #[indexed] balances_after: MultiValueEncoded<MultiValue3<TokenId, u64, BigUint>>,
    );
}
