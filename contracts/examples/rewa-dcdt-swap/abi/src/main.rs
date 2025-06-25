use rewa_dcdt_swap::*;
use numbat_wasm_debug::*;

fn main() {
	let contract = RewaDcdtSwapImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
