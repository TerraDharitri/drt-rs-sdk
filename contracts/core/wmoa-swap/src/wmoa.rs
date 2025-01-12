#![no_std]

dharitri_sc::imports!();

#[dharitri_sc::contract]
pub trait MoaDcdtSwap: dharitri_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_moa_token_id: TokenIdentifier) {
        self.wrapped_moa_token_id().set(&wrapped_moa_token_id);
    }

    // endpoints

    #[payable("MOA")]
    #[endpoint(wrapMoa)]
    fn wrap_moa(&self) -> DcdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().moa();
        require!(*payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();
        self.send()
            .dcdt_local_mint(&wrapped_moa_token_id, 0, &payment_amount);

        self.tx()
            .to(ToCaller)
            .single_dcdt(&wrapped_moa_token_id, 0, &payment_amount)
            .transfer();

        DcdtTokenPayment::new(wrapped_moa_token_id, 0, payment_amount.clone())
    }

    #[payable("*")]
    #[endpoint(unwrapMoa)]
    fn unwrap_moa(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_dcdt();
        let wrapped_moa_token_id = self.wrapped_moa_token_id().get();

        require!(*payment_token == wrapped_moa_token_id, "Wrong dcdt token");
        require!(*payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            *payment_amount <= self.get_locked_moa_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dcdt_local_burn(&wrapped_moa_token_id, 0, &payment_amount);

        // 1 wrapped MOA = 1 MOA, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.tx().to(&caller).moa(&*payment_amount).transfer();
    }

    #[view(getLockedMoaBalance)]
    #[title("lockedMoaBalance")]
    fn get_locked_moa_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&MoaOrDcdtTokenIdentifier::moa(), 0)
    }

    #[view(getWrappedMoaTokenId)]
    #[title("wrappedMoaTokenId")]
    #[storage_mapper("wrappedMoaTokenId")]
    fn wrapped_moa_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
