use super::big_uint_api_mock::*;
use crate::{TxContext, TxPanic};
use numbat_wasm::api::CallValueApi;
use numbat_wasm::err_msg;
use numbat_wasm::types::{DcdtTokenType, TokenIdentifier};

impl CallValueApi for TxContext {
	type AmountType = RustBigUint;

	fn check_not_payable(&self) {
		if self.rewa_value() > 0 {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_REWA.to_vec(),
			});
		}
		if self.dcdt_value() > 0 {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_DCDT.to_vec(),
			});
		}
	}

	#[inline]
	fn rewa_value(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn dcdt_value(&self) -> RustBigUint {
		self.tx_input_box.dcdt_value.clone().into()
	}

	#[inline]
	fn token(&self) -> TokenIdentifier {
		TokenIdentifier::from(self.tx_input_box.dcdt_token_identifier.as_slice())
	}

	#[inline]
	fn dcdt_token_nonce(&self) -> u64 {
		// TODO: Add DCDT nonce in mock
		0u64
	}

	#[inline]
	fn dcdt_token_type(&self) -> DcdtTokenType {
		// TODO: Add DCDT token type in mock
		DcdtTokenType::Fungible
	}
}
