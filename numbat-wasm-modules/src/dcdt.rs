use numbat_wasm::numbat_codec::TopEncode;

numbat_wasm::imports!();

/// Standard smart contract module for managing a single DCDT.
///
/// When added to a smart contract offers basic DCDT usage.
/// A lot of contracts use an owned DCDT for various purposes.
/// This module is used to offer a standard way of performing the basic operations.  
///
/// It provides endpoints for:
/// * issuing of an DCDT
/// * setting local roles
/// * minting/burning
///
#[numbat_wasm::module]
pub trait DcdtModule {
    /*
        DcdtTokenType is an enum (u8):
        0 - Fungible,
        1 - NonFungible,
        2 - SemiFungible,
        3 - Meta,

        Note: Only Fungible and Meta tokens have decimals
    */
    #[payable("REWA")]
    #[only_owner]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        #[payment_amount] issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: DcdtTokenType,
        #[var_args] opt_num_decimals: OptionalValue<usize>,
    ) {
        require!(self.token_id().is_empty(), "Token already issued");

        let num_decimals = match opt_num_decimals {
            OptionalValue::Some(d) => d,
            OptionalValue::None => 0,
        };

        self.send()
            .dcdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit()
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                // return payment to initial caller
                let initial_caller = self.blockchain().get_owner_address();
                let rewa_returned = self.call_value().rewa_value();
                if rewa_returned > 0u32 {
                    self.send()
                        .direct_rewa(&initial_caller, &rewa_returned, &[]);
                }
            },
        }
    }

    fn mint(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.token_id().get();
        self.send().dcdt_local_mint(&token_id, token_nonce, amount);
    }

    fn burn(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.token_id().get();
        self.send().dcdt_local_burn(&token_id, token_nonce, amount);
    }

    fn nft_create<T: TopEncode>(&self, amount: &BigUint, attributes: &T) -> u64 {
        let token_id = self.token_id().get();
        let empty_buffer = ManagedBuffer::new();
        let empty_vec = ManagedVec::from_raw_handle(empty_buffer.get_raw_handle());

        self.send().dcdt_nft_create(
            &token_id,
            amount,
            &empty_buffer,
            &BigUint::zero(),
            &empty_buffer,
            &attributes,
            &empty_vec,
        )
    }

    fn get_token_attributes<T: TopDecode>(&self, token_nonce: u64) -> T {
        let own_sc_address = self.blockchain().get_sc_address();
        let token_id = self.token_id().get();
        let token_data =
            self.blockchain()
                .get_dcdt_token_data(&own_sc_address, &token_id, token_nonce);

        token_data.decode_attributes()
    }

    fn require_token_issued(&self) {
        require!(!self.token_id().is_empty(), "Token must be issued first");
    }

    // Note: to issue another token, you have to clear this storage
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
