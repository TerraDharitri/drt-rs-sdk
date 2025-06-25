use super::{BigUintApi, ErrorApi};
use crate::err_msg;
use crate::types::{DcdtTokenType, TokenIdentifier};

pub trait CallValueApi<BigUint>: ErrorApi + Sized
where
	BigUint: BigUintApi + 'static,
{
	fn check_not_payable(&self);

	/// Retrieves the REWA call value from the VM.
	/// Will return 0 in case of an DCDT transfer (cannot have both REWA and DCDT transfer simultaneously).
	fn rewa_value(&self) -> BigUint;

	/// Retrieves the DCDT call value from the VM.
	/// Will return 0 in case of an REWA transfer (cannot have both REWA and DCDT transfer simultaneously).
	fn dcdt_value(&self) -> BigUint;

	/// Returns the call value token identifier of the current call.
	/// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
	///
	/// A note on implementation: even though the underlying api returns an empty name for REWA,
	/// but the REWA TokenIdentifier is serialized as `REWA`.
	fn token(&self) -> TokenIdentifier;

	/// Returns the nonce of the received DCDT token.
	/// Will return 0 in case of REWA or fungible DCDT transfer.
	fn dcdt_token_nonce(&self) -> u64;

	/// Returns the DCDT token type.
	/// Will return "Fungible" for REWA.
	fn dcdt_token_type(&self) -> DcdtTokenType;

	/// Will return the REWA call value,
	/// but also fail with an error if DCDT is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_rewa(&self) -> BigUint {
		if !self.token().is_rewa() {
			self.signal_error(err_msg::NON_PAYABLE_FUNC_DCDT);
		}
		self.rewa_value()
	}

	/// Will return the DCDT call value,
	/// but also fail with an error if REWA or the wrong DCDT token is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_dcdt(&self, token: &[u8]) -> BigUint {
		if self.token() != token {
			self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
		}
		self.dcdt_value()
	}

	/// Returns both the call value (either REWA or DCDT) and the token identifier.
	/// Especially used in the `#[payable("*")] auto-generated snippets.
	/// The method might seem redundant, but there is such a hook in Andes
	/// that might be used in this scenario in the future.
	fn payment_token_pair(&self) -> (BigUint, TokenIdentifier) {
		let token = self.token();
		if token.is_rewa() {
			(self.rewa_value(), token)
		} else {
			(self.dcdt_value(), token)
		}
	}
}
