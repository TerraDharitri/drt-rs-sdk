use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/rewa-dcdt-swap.wasm",
		Box::new(|context| Box::new(rewa_dcdt_swap::contract_obj(context))),
	);
	contract_map
}

#[test]
fn unwrap_rewa_rs() {
	numbat_wasm_debug::denali_rs("denali/unwrap_rewa.scen.json", &contract_map());
}

#[test]
fn wrap_rewa_rs() {
	numbat_wasm_debug::denali_rs("denali/wrap_rewa.scen.json", &contract_map());
}
