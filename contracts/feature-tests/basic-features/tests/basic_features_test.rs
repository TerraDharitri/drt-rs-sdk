use basic_features::*;
use numbat_wasm::types::{SCError, SCResult};
use numbat_wasm_debug::*;

#[test]
fn test_sc_error() {
	let bf = BasicFeaturesImpl::new(TxContext::dummy());
	let result = bf.return_sc_error();
	assert_eq!(
		SCResult::Err(SCError::from(&b"return_sc_error"[..])),
		result
	);
}
