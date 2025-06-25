#![no_std]
#![allow(unused_attributes)]

numbat_wasm::imports!();

const REWA_DECIMALS: usize = 18;

#[numbat_wasm_derive::contract]
pub trait Child {
	#[init]
	fn init(&self) {}

	#[payable("REWA")]
	#[endpoint(issueWrappedRewa)]
	fn issue_wrapped_rewa(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: Self::BigUint,
		#[payment] issue_cost: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		DCDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: REWA_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: false,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().dcdt_issue_callback())
	}

	// callbacks

	#[callback]
	fn dcdt_issue_callback(
		&self,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] _amount: Self::BigUint,
		#[call_result] _result: AsyncCallResult<()>,
	) {
		self.wrapped_rewa_token_identifier().set(&token_identifier);
	}

	// storage

	#[view(getWrappedRewaTokenIdentifier)]
	#[storage_mapper("wrappedRewaTokenIdentifier")]
	fn wrapped_rewa_token_identifier(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
