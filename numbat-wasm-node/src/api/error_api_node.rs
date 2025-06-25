use crate::error_hook;
use crate::AndesApiImpl;
use numbat_wasm::api::ErrorApi;

impl ErrorApi for AndesApiImpl {
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		error_hook::signal_error(message)
	}
}
