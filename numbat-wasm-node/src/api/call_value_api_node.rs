use super::AndesBigUint;
use crate::AndesApiImpl;
use numbat_wasm::api::CallValueApi;
use numbat_wasm::types::{BoxedBytes, DcdtTokenType, TokenIdentifier};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
	fn checkNoPayment();

	fn bigIntNew(value: i64) -> i32;

	fn bigIntGetCallValue(dest: i32);
	fn bigIntGetDCDTCallValue(dest: i32);
	fn getDCDTTokenName(resultOffset: *const u8) -> i32;
	fn getDCDTTokenNonce() -> u64;
	fn getDCDTTokenType() -> i32;

	/// TODO: decide if it is worth using or not
	#[allow(dead_code)]
	fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for AndesApiImpl {
	type AmountType = AndesBigUint;

	#[inline]
	fn check_not_payable(&self) {
		unsafe {
			checkNoPayment();
		}
	}

	fn rewa_value(&self) -> AndesBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetCallValue(result);
			AndesBigUint { handle: result }
		}
	}

	fn dcdt_value(&self) -> AndesBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetDCDTCallValue(result);
			AndesBigUint { handle: result }
		}
	}

	fn token(&self) -> TokenIdentifier {
		unsafe {
			let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
			let name_len = getDCDTTokenName(name_buffer.as_mut_ptr());
			if name_len == 0 {
				TokenIdentifier::rewa()
			} else {
				BoxedBytes::from(&name_buffer[..name_len as usize]).into()
			}
		}
	}

	fn dcdt_token_nonce(&self) -> u64 {
		unsafe { getDCDTTokenNonce() }
	}

	fn dcdt_token_type(&self) -> DcdtTokenType {
		unsafe { (getDCDTTokenType() as u8).into() }
	}
}
