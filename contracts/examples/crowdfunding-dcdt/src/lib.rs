#![no_std]
#![allow(unused_attributes)]

imports!();
derive_imports!();

use numbat_wasm::HexCallDataSerializer;

const DCDT_TRANSFER_STRING: &[u8] = b"DCDTTransfer";

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[numbat_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: BigUint, deadline: u64, dcdt_token_name: BoxedBytes) {
		let my_address: Address = self.get_caller();
		self.set_owner(&my_address);
		self.set_target(&target);
		self.set_deadline(deadline);
		self.set_cf_dcdt_token_name(&dcdt_token_name);
	}

	#[endpoint]
	fn fund(&self) -> SCResult<()> {
		if self.get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		let expected_token_name = self.get_cf_dcdt_token_name();
		let actual_token_name = self.get_dcdt_token_name_boxed();

		if expected_token_name != actual_token_name {
			return sc_error!("wrong dcdt token");
		}

		let payment = self.get_dcdt_value_big_uint();
		let caller = self.get_caller();
		let mut deposit = self.get_deposit(&caller);
		let mut balance = self.get_dcdt_balance();

		deposit += payment.clone();
		balance += payment;

		self.set_deposit(&caller, &deposit);
		self.set_dcdt_balance(&balance);

		Ok(())
	}

	#[view]
	fn status(&self) -> Status {
		if self.get_block_nonce() <= self.get_deadline() {
			Status::FundingPeriod
		} else if self.get_dcdt_balance() >= self.get_target() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[view(currentFunds)]
	fn current_funds(&self) -> SCResult<BigUint> {
		Ok(self.get_dcdt_balance())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.get_caller();
				if caller != self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}

				let dcdt_token_name = self.get_cf_dcdt_token_name();
				let dcdt_balance = self.get_dcdt_balance();

				self.set_dcdt_balance(&BigUint::zero());
				self.pay_dcdt(&dcdt_token_name, &dcdt_balance, &caller);

				Ok(())
			},
			Status::Failed => {
				let caller = self.get_caller();
				let deposit = self.get_deposit(&caller);

				if deposit > 0 {
					let dcdt_token_name = self.get_cf_dcdt_token_name();
					let mut dcdt_balance = self.get_dcdt_balance();

					dcdt_balance -= deposit.clone();

					self.set_dcdt_balance(&dcdt_balance);
					self.set_deposit(&caller, &BigUint::zero());
					self.pay_dcdt(&dcdt_token_name, &deposit, &caller);
				}
				Ok(())
			},
		}
	}

	fn get_dcdt_token_name_boxed(&self) -> BoxedBytes {
		BoxedBytes::from(self.get_dcdt_token_name())
	}

	fn pay_dcdt(&self, dcdt_token_name: &BoxedBytes, amount: &BigUint, to: &Address) {
		let mut serializer = HexCallDataSerializer::new(DCDT_TRANSFER_STRING);
		serializer.push_argument_bytes(dcdt_token_name.as_slice());
		serializer.push_argument_bytes(amount.to_bytes_be().as_slice());

		self.async_call(&to, &BigUint::zero(), serializer.as_slice());
	}

	// storage

	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("target")]
	fn set_target(&self, target: &BigUint);

	#[view]
	#[storage_get("target")]
	fn get_target(&self) -> BigUint;

	#[storage_set("dcdtBalance")]
	fn set_dcdt_balance(&self, dcdt_balance: &BigUint);

	#[view]
	#[storage_get("dcdtBalance")]
	fn get_dcdt_balance(&self) -> BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, amount: &BigUint);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> BigUint;

	#[storage_set("dcdtTokenName")]
	fn set_cf_dcdt_token_name(&self, dcdt_token_name: &BoxedBytes);

	#[view]
	#[storage_get("dcdtTokenName")]
	fn get_cf_dcdt_token_name(&self) -> BoxedBytes;
}
