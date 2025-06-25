#![no_std]
#![allow(unused_attributes)]

numbat_wasm::imports!();

use numbat_wasm::HexCallDataSerializer;

const DCDT_TRANSFER_STRING: &[u8] = b"DCDTTransfer";
const SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT: &[u8] = b"acceptDcdtPayment";
const SECOND_CONTRACT_REJECT_DCDT_PAYMENT: &[u8] = b"rejectDcdtPayment";

#[numbat_wasm_derive::contract(FirstContractImpl)]
pub trait FirstContract {
	#[init]
	fn init(&self, dcdt_token_name: TokenIdentifier, second_contract_address: Address) {
		self.set_contract_dcdt_token_name(&dcdt_token_name);
		self.set_second_contract_address(&second_contract_address);
	}

	#[payable("*")]
	#[endpoint(transferToSecondContractFull)]
	fn transfer_to_second_contract_full(
		&self,
		#[payment] dcdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_dcdt_token_name();

		require!(dcdt_value > 0, "no dcdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong dcdt token");

		self.call_dcdt_second_contract(
			&expected_token_name,
			&dcdt_value,
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(transferToSecondContractHalf)]
	fn transfer_to_second_contract_half(
		&self,
		#[payment] dcdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_dcdt_token_name();

		require!(dcdt_value > 0, "no dcdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong dcdt token");

		self.call_dcdt_second_contract(
			&expected_token_name,
			&(dcdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_ACCEPT_DCDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	#[payable("*")]
	#[endpoint]
	fn transfer_to_second_contract_rejected(
		&self,
		#[payment] dcdt_value: BigUint,
		#[payment_token] actual_token_name: TokenIdentifier,
	) -> SCResult<()> {
		let expected_token_name = self.get_contract_dcdt_token_name();

		require!(dcdt_value > 0, "no dcdt transfered!");
		require!(actual_token_name == expected_token_name, "Wrong dcdt token");

		self.call_dcdt_second_contract(
			&expected_token_name,
			&(dcdt_value / BigUint::from(2u32)),
			&self.get_second_contract_address(),
			SECOND_CONTRACT_REJECT_DCDT_PAYMENT,
			&[],
		);

		Ok(())
	}

	fn call_dcdt_second_contract(
		&self,
		dcdt_token_name: &TokenIdentifier,
		amount: &BigUint,
		to: &Address,
		func_name: &[u8],
		args: &[BoxedBytes],
	) {
		let mut serializer = HexCallDataSerializer::new(DCDT_TRANSFER_STRING);
		serializer.push_argument_bytes(dcdt_token_name.as_dcdt_identifier());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
		serializer.push_argument_bytes(func_name);
		for arg in args {
			serializer.push_argument_bytes(arg.as_slice());
		}

		self.send()
			.async_call_raw(&to, &BigUint::zero(), serializer.as_slice());
	}

	// storage

	#[storage_set("dcdtTokenName")]
	fn set_contract_dcdt_token_name(&self, dcdt_token_name: &TokenIdentifier);

	#[view(getDcdtTokenName)]
	#[storage_get("dcdtTokenName")]
	fn get_contract_dcdt_token_name(&self) -> TokenIdentifier;

	#[storage_set("secondContractAddress")]
	fn set_second_contract_address(&self, address: &Address);

	#[view(getSecondContractAddress)]
	#[storage_get("secondContractAddress")]
	fn get_second_contract_address(&self) -> Address;
}
