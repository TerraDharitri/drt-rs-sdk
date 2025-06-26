dharitri_sc::imports!();

/// Not directly related to promises, but this contract already has the setup for VM 1.5.
#[dharitri_sc::module]
pub trait BackTransfersFeatureModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn forward_sync_retrieve_funds_bt(
        &self,
        to: ManagedAddress,
        token: RewaOrDcdtTokenIdentifier,
        token_nonce: u64,
        amount: BigUint,
    ) {
        let ((), back_transfers) = self
            .vault_proxy()
            .contract(to)
            .retrieve_funds(token, token_nonce, amount)
            .execute_on_dest_context_with_back_transfers::<()>();

        self.back_transfers_event(
            &back_transfers.total_rewa_amount,
            &back_transfers.dcdt_payments.into_multi_value(),
        );
    }

    #[event("back_tranfers")]
    fn back_transfers_event(
        &self,
        #[indexed] rewa_value: &BigUint,
        #[indexed] multi_dcdt: &MultiValueEncoded<DcdtTokenPaymentMultiValue>,
    );
}
