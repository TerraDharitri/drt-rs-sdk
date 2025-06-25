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
    fn issue_wrapped_rewa(
        &self,
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: Self::BigUint,
        #[payment] issue_cost: Self::BigUint,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(
            self.wrapped_rewa_token_id().is_empty(),
            "wrapped rewa was already issued"
        );

        let caller = self.blockchain().get_caller();

        self.issue_started_event(&caller, token_ticker.as_slice(), &initial_supply);

        Ok(DCDTSystemSmartContractProxy::new_proxy_obj(self.send())
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
            .with_callback(self.callbacks().dcdt_issue_callback(&caller)))
    }

    #[callback]
    fn dcdt_issue_callback(
        &self,
        caller: &Address,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] returned_tokens: Self::BigUint,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        // callback is called with DCDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            AsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.unused_wrapped_rewa().set(&returned_tokens);
                self.wrapped_rewa_token_id().set(&token_identifier);
            },
            AsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, message.err_msg.as_slice());

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_rewa() && returned_tokens > 0 {
                    self.send().direct_rewa(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[only_owner]
    #[endpoint(mintWrappedRewa)]
    fn mint_wrapped_rewa(&self, amount: Self::BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(
            !self.wrapped_rewa_token_id().is_empty(),
            "Wrapped REWA was not issued yet"
        );

        let wrapped_rewa_token_id = self.wrapped_rewa_token_id().get();
        let caller = self.blockchain().get_caller();
        self.mint_started_event(&caller, &amount);

        Ok(DCDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .mint(&wrapped_rewa_token_id, &amount)
            .async_call()
            .with_callback(self.callbacks().dcdt_mint_callback(&caller, &amount)))
    }

    #[callback]
    fn dcdt_mint_callback(
        &self,
        caller: &Address,
        amount: &Self::BigUint,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        match result {
            AsyncCallResult::Ok(()) => {
                self.mint_success_event(caller);
                self.unused_wrapped_rewa()
                    .update(|unused_wrapped_rewa| *unused_wrapped_rewa += amount);
            },
            AsyncCallResult::Err(message) => {
                self.mint_failure_event(caller, message.err_msg.as_slice());
            },
        }
    }

    // endpoints

    #[payable("REWA")]
    #[endpoint(wrapRewa)]
    fn wrap_rewa(&self, #[payment] payment: Self::BigUint) -> SCResult<()> {
        require!(payment > 0, "Payment must be more than 0");
        require!(
            !self.wrapped_rewa_token_id().is_empty(),
            "Wrapped REWA was not issued yet"
        );

        self.unused_wrapped_rewa().update(|unused_wrapped_rewa| {
            require!(
                *unused_wrapped_rewa > payment,
                "Contract does not have enough wrapped REWA. Please try again once more is minted."
            );

            *unused_wrapped_rewa -= &payment;

            Ok(())
        })?;

        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &self.wrapped_rewa_token_id().get(),
            0,
            &payment,
            b"wrapping",
        );

        self.wrap_rewa_event(&caller, &payment);

        Ok(())
    }

    #[payable("*")]
    #[endpoint(unwrapRewa)]
    fn unwrap_rewa(
        &self,
        #[payment] wrapped_rewa_payment: Self::BigUint,
        #[payment_token] token_identifier: TokenIdentifier,
    ) -> SCResult<()> {
        require!(
            !self.wrapped_rewa_token_id().is_empty(),
            "Wrapped REWA was not issued yet"
        );

        let wrapped_rewa_token_identifier = self.wrapped_rewa_token_id().get();
        require!(
            token_identifier == wrapped_rewa_token_identifier,
            "Wrong dcdt token"
        );

        require!(wrapped_rewa_payment > 0, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            wrapped_rewa_payment
                <= self
                    .blockchain()
                    .get_sc_balance(&TokenIdentifier::rewa(), 0),
            "Contract does not have enough funds"
        );

        self.unused_wrapped_rewa()
            .update(|unused_wrapped_rewa| *unused_wrapped_rewa += &wrapped_rewa_payment);

        // 1 wrapped REWA = 1 REWA, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_rewa(&caller, &wrapped_rewa_payment, b"unwrapping");

        self.unwrap_rewa_event(&caller, &wrapped_rewa_payment);

        Ok(())
    }

    #[view(getLockedRewaBalance)]
    fn get_locked_rewa_balance(&self) -> Self::BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::rewa(), 0)
    }

    // storage

    #[view(getWrappedRewaTokenIdentifier)]
    #[storage_mapper("wrapped_rewa_token_id")]
    fn wrapped_rewa_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getUnusedWrappedRewa)]
    #[storage_mapper("unused_wrapped_rewa")]
    fn unused_wrapped_rewa(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] token_ticker: &[u8],
        initial_supply: &Self::BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &Self::BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

    #[event("mint-started")]
    fn mint_started_event(&self, #[indexed] caller: &Address, amount: &Self::BigUint);

    #[event("mint-success")]
    fn mint_success_event(&self, #[indexed] caller: &Address);

    #[event("mint-failure")]
    fn mint_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

    #[event("wrap-rewa")]
    fn wrap_rewa_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

    #[event("unwrap-rewa")]
    fn unwrap_rewa_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}
