#![no_std]

numbat_wasm::imports!();
numbat_wasm::derive_imports!();

const REWA_NUM_DECIMALS: usize = 18;

/// Converts between REWA and a wrapped REWA DCDT token.
///	1 REWA = 1 wrapped REWA and is interchangeable at all times.
/// Also manages the supply of wrapped REWA tokens.
#[numbat_wasm::contract]
pub trait RewaDcdtSwap {
    #[init]
    fn init(&self) {}

    // endpoints - owner-only

    #[only_owner]
    #[payable("REWA")]
    #[endpoint(issueWrappedRewa)]
    fn issue_wrapped_rewa(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(
            self.wrapped_rewa_token_id().is_empty(),
            "wrapped rewa was already issued"
        );

        let issue_cost = self.call_value().rewa_value();
        let caller = self.blockchain().get_caller();
        let initial_supply = BigUint::zero();

        self.issue_started_event(&caller, &token_ticker, &initial_supply);

        self.send()
            .dcdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: REWA_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().dcdt_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn dcdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();

        // callback is called with DCDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.wrapped_rewa_token_id().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, &message.err_msg);

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_rewa() && returned_tokens > 0 {
                    self.send().direct_rewa(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(
            !self.wrapped_rewa_token_id().is_empty(),
            "Must issue token first"
        );

        let roles = [DcdtLocalRole::Mint, DcdtLocalRole::Burn];
        self.send()
            .dcdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.wrapped_rewa_token_id().get(),
                roles[..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    // endpoints

    #[payable("REWA")]
    #[endpoint(wrapRewa)]
    fn wrap_rewa(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        require!(payment_token.is_rewa(), "Only REWA accepted");
        require!(payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_rewa_token_id = self.wrapped_rewa_token_id().get();
        self.send()
            .dcdt_local_mint(&wrapped_rewa_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct(&caller, &wrapped_rewa_token_id, 0, &payment_amount, &[]);
    }

    #[payable("*")]
    #[endpoint(unwrapRewa)]
    fn unwrap_rewa(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let wrapped_rewa_token_id = self.wrapped_rewa_token_id().get();

        require!(payment_token == wrapped_rewa_token_id, "Wrong dcdt token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            payment_amount <= self.get_locked_rewa_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dcdt_local_burn(&wrapped_rewa_token_id, 0, &payment_amount);

        // 1 wrapped rEWA = 1 rEWA, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_rewa(&caller, &payment_amount, &[]);
    }

    #[view(getLockedRewaBalance)]
    fn get_locked_rewa_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::rewa(), 0)
    }

    // storage

    #[view(getWrappedRewaTokenIdentifier)]
    #[storage_mapper("wrappedRewaTokenId")]
    fn wrapped_rewa_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

    #[event("wrap-rewa")]
    fn wrap_rewa_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("unwrap-rewa")]
    fn unwrap_rewa_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
