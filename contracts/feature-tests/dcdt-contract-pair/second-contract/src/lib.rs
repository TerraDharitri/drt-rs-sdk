#![no_std]
#![allow(unused_attributes)]

numbat_wasm::imports!();

#[numbat_wasm_derive::contract]
pub trait SecondContract {
	#[init]
	fn init(&self, dcdt_token_name: TokenIdentifier) {
		self.set_contract_dcdt_token_name(&dcdt_token_name);
	}

	#[payable("*")]
	#[endpoint(acceptDcdtPayment)]
	fn accept_dcdt_payment(
		&self,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_dcdt_token_name();
		require!(actual_token_name == expected_token_name, "Wrong dcdt token");
		Ok(())
	}

	#[payable("*")]
	#[endpoint(rejectDcdtPayment)]
	fn reject_dcdt_payment(&self) -> SCResult<()> {
		sc_error!("Rejected")
	}

	// storage

	#[storage_set("dcdtTokenName")]
	fn set_contract_dcdt_token_name(&self, dcdt_token_name: &TokenIdentifier);

	#[view(getDcdtTokenName)]
	#[storage_get("dcdtTokenName")]
	fn get_contract_dcdt_token_name(&self) -> TokenIdentifier;
}
