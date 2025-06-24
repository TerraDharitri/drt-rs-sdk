use abi_tester::*;
use numbat_wasm_debug::*;

fn main() {
	let contract = AbiTesterImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
